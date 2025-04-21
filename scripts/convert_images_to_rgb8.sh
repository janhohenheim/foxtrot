#!/usr/bin/env bash

set -euo pipefail
shopt -s globstar nullglob

readonly ASSETS_PATH='assets'

main() {
    for file in "${ASSETS_PATH}"/**/*.png; do
        magick "${file}" -depth 8 "${file}"
    done
}

main "$@"
exit "$?"
