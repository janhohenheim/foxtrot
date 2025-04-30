#!/usr/bin/env bash

set -euo pipefail
shopt -s globstar nullglob

# Convert only files in this directory
readonly TEXTURES_PATH='assets/textures'
# Convert only these file extensions
readonly FILE_EXTENSIONS=('jpg' 'jpeg' 'png' 'tga')
# Darkmod textures end with this suffix
readonly DARKMOD_SUFFIX='_ed.png'

main() {
    for extension in "${FILE_EXTENSIONS[@]}"; do
        for file in "${TEXTURES_PATH}"/**/*.${extension}; do
            # convert to PNG with rgb8
            magick "${file}" -depth 8 "${file%.${extension}}.png"
            # remove the original file if it was converted
            if [[ "${file%.${extension}}.png" != "${file}" ]]; then
                rm "${file}"
            fi
            # Create a material file if the texture is a darkmod texture that ends with _ed
            if [[ "${file}" == *"${DARKMOD_SUFFIX}" ]]; then
                # Remove the _ed suffix from the file name
                # We do this for the following reasons:
                # - The material file needs to have the same name as the base texture
                # - The normal texture is named after the name of the base texture plus the suffix _local
                # Since the normal texture does not have the _ed suffix, we need to remove it from the base texture name
                local name_without_suffix="${file%${DARKMOD_SUFFIX}}"
                mv "${file}" "${name_without_suffix}.png"

                # create the material file
                echo "inherits = \"/textures/darkmod.toml\"" > "${name_without_suffix}.toml"
            fi
        done
    done
}

main "$@"
exit "$?"
