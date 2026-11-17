import assert from 'node:assert/strict';
import { readFile, readdir } from 'node:fs/promises';
import path from 'node:path';
import { JSDOM } from 'jsdom';

const root = path.resolve(process.argv[2] || 'site');
let posts = 0;
let equations = 0;
const failures = [];
async function check(directory) {
  for (const entry of await readdir(directory, { withFileTypes: true })) {
    const filename = path.join(directory, entry.name);
    if (entry.isDirectory()) {
      await check(filename);
      continue;
    }
    if (!entry.name.endsWith('.html')) continue;
    const relative = path.relative(root, filename);
    const dom = new JSDOM(await readFile(filename, 'utf8'));
    const document = dom.window.document;
    if (/^\d{4}[/\\]/.test(relative)) posts++;
    equations += document.querySelectorAll('mjx-container').length;
    if (document.querySelector('script, iframe')) failures.push(`${relative}: client-side script or iframe`);
    if (document.querySelector('merror, [data-mjx-error]')) failures.push(`${relative}: math rendering error`);
    const content = document.querySelector('.content');
    if (content) {
      // Leave code examples and accessible copies of already-rendered equations alone.
      content.querySelectorAll('pre, code, mjx-container, math, svg, style, [data-tex-escaped]').forEach(node => node.remove());
      const raw = content.textContent.match(/\$\$[\s\S]+?\$\$|\$(?=\S)[^$]*\S\$|\\\([\s\S]+?\\\)|\\\[[\s\S]+?\\\]/g);
      if (raw) failures.push(`${relative}: unrendered math: ${raw.join(', ')}`);
    }
    dom.window.close();
  }
}
await check(root);
assert.equal(failures.length, 0, failures.join('\n'));
console.log(`Audited ${posts} posts and ${equations} rendered equations: no unrendered math or client-side JavaScript.`);
