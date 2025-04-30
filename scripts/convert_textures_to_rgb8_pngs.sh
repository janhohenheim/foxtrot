#!/usr/bin/env bash

set -euo pipefail
shopt -s globstar nullglob

readonly TEXTURES_PATH='assets/textures'
readonly FILE_EXTENSIONS=('jpg' 'jpeg' 'png' 'tga')

main() {
    for extension in "${FILE_EXTENSIONS[@]}"; do
        for file in "${TEXTURES_PATH}"/**/*.${extension}; do
            # convert to PNG
            magick "${file}" -depth 8 "${file%.${extension}}.png"
            # remove the original file if it was converted
            if [[ "${file%.${extension}}.png" != "${file}" ]]; then
                rm "${file}"
            fi
        done
    done
}

main "$@"
exit "$?"
