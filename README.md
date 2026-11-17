kaashif's blog
==============

These files are turned into a website by a small Rust program, tau.

To build the site, install stable Rust, a C compiler, make, Node.js, and Pygments,
then run `npm ci` and `./tau regen`. `tests/parity.sh` checks the generated files
byte-for-byte against output from the old Perl implementation, ignoring
filesystem-dependent URL ordering in the sitemap.

See the finished product at <https://www.kaashif.co.uk>.

The build renders TeX equations to SVG with accessible MathML and reserves
space for local images. Text uses Helvetica/Arial system fonts, with no
webfont downloads or font swaps.
Published pages need no JavaScript. Disqus is no longer included.
CSS reveals the page when the complete HTML has arrived, avoiding partially
painted paragraphs while the browser is still parsing the response.
The combined stylesheet has a content hash and long-lived browser caching;
changing the CSS gives it a new URL automatically.
Run `node scripts/check-site.mjs site` to audit every generated post for
unrendered math and unwanted scripts. Deployment runs this check automatically.

Footnotes
---------

Posts and pages support Markdown footnotes, with numbered references and
links back to the text:

```markdown
An aside belongs in a footnote.[^aside]

[^aside]: Footnotes can include **formatting** and [links](https://example.com).

    Indent continuation paragraphs by four spaces.
```

License
-------
The code in this repository (that is, the code snippets contained in
blog posts and the `tau` script itself) is licensed under the ISC
license, there is a copy in the `CODE_LICENSE` file.

The non-code content in this repository: blog posts, about pages,
contact pages, config files, and anything else that isn't runnable or
compilable code, is licensed under the CC BY-NC license, contained in
the `LICENSE` file.
