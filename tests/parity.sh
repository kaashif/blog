#!/bin/sh
set -eu

repo=$(CDPATH= cd -- "$(dirname "$0")/.." && pwd)
fixture=$(mktemp -d "${TMPDIR:-/tmp}/tau-parity.XXXXXX")
trap 'rm -rf "$fixture"' EXIT HUP INT TERM

cp -R "$repo/tests/fixture/blog/." "$fixture/"
"$repo/target/release/tau" --root "$fixture" regen >/dev/null
diff -r "$repo/tests/fixture/expected" "$fixture/site"
