#!/bin/sh
set -eu

repo=$(CDPATH= cd -- "$(dirname "$0")/.." && pwd)
fixture=$(mktemp -d "${TMPDIR:-/tmp}/tau-parity.XXXXXX")
trap 'rm -rf "$fixture"' EXIT HUP INT TERM

cp -R "$repo/tests/fixture/blog/." "$fixture/"
"$repo/target/release/tau" --root "$fixture" regen >/dev/null
# Directory iteration order differs between filesystems. Sitemap URL order has
# no meaning, so compare its contents while keeping every other byte exact.
cp -R "$repo/tests/fixture/expected" "$fixture/expected"
LC_ALL=C sort "$fixture/expected/sitemap.txt" -o "$fixture/expected/sitemap.txt"
LC_ALL=C sort "$fixture/site/sitemap.txt" -o "$fixture/site/sitemap.txt"
diff -r "$fixture/expected" "$fixture/site"
