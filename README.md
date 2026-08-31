kaashif's blog
==============

These files are turned into a website by a small Rust program, tau.

To build the site, install stable Rust, a C compiler, make, and Pygments,
then run `./tau regen`. `tests/parity.sh` checks the generated files
byte-for-byte against output from the old Perl implementation.

See the finished product at <https://www.kaashif.co.uk>.

License
-------
The code in this repository (that is, the code snippets contained in
blog posts and the `tau` script itself) is licensed under the ISC
license, there is a copy in the `CODE_LICENSE` file.

The non-code content in this repository: blog posts, about pages,
contact pages, config files, and anything else that isn't runnable or
compilable code, is licensed under the CC BY-NC license, contained in
the `LICENSE` file.
