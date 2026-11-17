import { readFile, writeFile, readdir } from 'node:fs/promises';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { mathjax } from '@mathjax/src/js/mathjax.js';
import { TeX } from '@mathjax/src/js/input/tex.js';
import { SVG } from '@mathjax/src/js/output/svg.js';
import { jsdomAdaptor } from '@mathjax/src/js/adaptors/jsdomAdaptor.js';
import { JSDOM } from 'jsdom';
import { RegisterHTMLHandler } from '@mathjax/src/js/handlers/html.js';
import { AssistiveMmlHandler } from '@mathjax/src/js/a11y/assistive-mml.js';
import '@mathjax/src/js/util/asyncLoad/esm.js';
import '@mathjax/src/js/input/tex/base/BaseConfiguration.js';
import '@mathjax/src/js/input/tex/ams/AmsConfiguration.js';
import '@mathjax/src/js/input/tex/newcommand/NewcommandConfiguration.js';
import { imageSize } from 'image-size';

const root = path.resolve(process.argv[2] || 'site');
const adaptor = jsdomAdaptor(JSDOM, { fontSize: 16 });
AssistiveMmlHandler(RegisterHTMLHandler(adaptor));

// One cacheable stylesheet; text uses system fonts with no font downloads.
let css = await readFile(path.join(root, 'css/style.css'), 'utf8');
css += '\n' + await readFile(path.join(root, 'css/monokai.css'), 'utf8');
await writeFile(path.join(root, 'css/style.css'), css);
const cssName = `style.${createHash('sha256').update(css).digest('hex').slice(0, 16)}.css`;
await writeFile(path.join(root, 'css', cssName), css);
await writeFile(path.join(root, '_headers'), `/css/${cssName}\n  Cache-Control: public, max-age=31536000, immutable\n`);

async function* htmlFiles(directory) {
  for (const entry of await readdir(directory, { withFileTypes: true })) {
    const filename = path.join(directory, entry.name);
    if (entry.isDirectory()) yield* htmlFiles(filename);
    else if (entry.name.endsWith('.html')) yield filename;
  }
}

let equations = 0;
let pages = 0;
const errors = [];
for await (const filename of htmlFiles(root)) {
  const source = (await readFile(filename, 'utf8')).replace('/css/style.css', `/css/${cssName}`);
  let parsed;
  try {
    parsed = await mathjax.handleRetriesFor(() => adaptor.parse(source, 'text/html'));
  } catch (error) {
    throw new Error(`Parsing ${path.relative(root, filename)}: ${error.message}`);
  }
  const document = mathjax.document(parsed, {
    InputJax: new TeX({
      packages: ['base', 'ams', 'newcommand'],
      inlineMath: [['$', '$'], ['\\(', '\\)']],
      processEscapes: true,
      formatError(jax, error) {
        errors.push(`${path.relative(root, filename)}: ${error.message}`);
        return jax.formatError(error);
      },
    }),
    OutputJax: new SVG({ fontCache: 'global' }),
  });
  await document.renderPromise();
  for (const item of document.math) {
    if (item.isEscaped && item.typesetRoot) {
      adaptor.setAttribute(item.typesetRoot, 'data-tex-escaped', '');
    }
  }
  const count = Array.from(document.math).filter(item => !item.isEscaped).length;
  equations += count;
  if (count) pages++;
  // Also parse pages without equations to add intrinsic image dimensions.
  for (const image of adaptor.tags(adaptor.body(document.document), 'img')) {
    const src = adaptor.getAttribute(image, 'src');
    if (!src || /^(?:https?:|data:|\/\/)/.test(src)) continue;
    const location = src.startsWith('/')
      ? path.join(root, src.split(/[?#]/)[0])
      : path.resolve(path.dirname(filename), src.split(/[?#]/)[0]);
    try {
      const { width, height } = imageSize(await readFile(location));
      if (!adaptor.hasAttribute(image, 'width') && !adaptor.hasAttribute(image, 'height')) {
        adaptor.setAttribute(image, 'width', width);
        adaptor.setAttribute(image, 'height', height);
      }
    } catch (error) {
      // Some old posts link to assets that are no longer in the repository.
      if (error.code !== 'ENOENT') throw error;
    }
  }
  if (!count) {
    const style = document.outputJax.svgStyles;
    if (style && adaptor.parent(style)) adaptor.remove(style);
  }
  const body = adaptor.body(document.document);
  const ready = document.document.getElementById('page-ready');
  if (ready) adaptor.append(body, ready);
  const output = adaptor.doctype(document.document) + '\n' + adaptor.outerHTML(adaptor.root(document.document));
  if (/<merror\b/.test(output)) errors.push(`${path.relative(root, filename)}: invalid rendered equation`);
  await writeFile(filename, output);
}
if (errors.length) throw new Error('Math rendering failed:\n' + errors.join('\n'));
console.log(`Rendered ${equations} equations across ${pages} pages; no math errors.`);
