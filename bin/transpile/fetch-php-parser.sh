#!/usr/bin/env bash
# Fetches the nikic/php-parser sources (with tests) at the installed version into build/php-parser.
set -euo pipefail
cd "$(dirname "$0")/../.."
version=$(composer show nikic/php-parser 2>/dev/null | sed -nE 's/^versions +: \* (v[0-9.]+).*/\1/p')
[ -n "$version" ] || { echo "could not determine installed php-parser version" >&2; exit 1; }
if [ -d build/php-parser ] && [ "$(cat build/php-parser/.fetched-version 2>/dev/null)" = "$version" ]; then
    echo "build/php-parser already at $version"; exit 0
fi
rm -rf build/php-parser
git clone -q --depth 1 --branch "$version" https://github.com/nikic/PHP-Parser.git build/php-parser
echo "$version" > build/php-parser/.fetched-version
echo "fetched php-parser $version"
