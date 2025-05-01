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
            if [[ "${file%.${extension}}.png" != "${file}" ]]; then
                # convert to PNG with rgb8
                magick "${file}" -depth 8 "${file%.${extension}}.png"
                # remove the unconverted file
                rm "${file}"
            else
                # These are already PNGs, so let's see if we need to adjust the depth
                local colorspace=$(identify -format "%[colorspace]" "${file}")
                local depth=$(identify -format "%[depth]" "${file}")
                # We allow only 2, 4, 8 as depths and only sRGB and Gray as colorspaces
                # I believe this is what Bevy supports? If we see crashes when loading the textures, we can try other depths
                local allowed_depths=('2' '4' '8')
                local allowed_colorspaces=('sRGB' 'Gray')
                if [[ ! " ${allowed_depths[@]} " =~ " ${depth} " ]] || [[ ! " ${allowed_colorspaces[@]} " =~ " ${colorspace} " ]]; then
                    magick "${file}" -depth 8 "${file}"
                fi
            fi
            # file now has to have a .png extension
            file="${file%.${extension}}.png"

            # if the file ends in _local.png, rename it to _normal.png
            if [[ "${file}" == *"_local.png" ]]; then
                mv "${file}" "${file%_local.png}_normal.png"
            fi

            local stripped_name=$(basename "${file}" ".png")
            local non_base_suffixes=('_local' '_disp' '_arm' '_nor')
            local has_suffix=false
            for suffix in "${non_base_suffixes[@]}"; do
                if [[ "$stripped_name" == *"$suffix"* ]]; then
                    has_suffix=true
                    break
                fi
            done
            if [[ "$has_suffix" == false ]]; then
                # The quake 1 map format only supports 16 character file names
                if [[ "${stripped_name}" =~ ^.{16,}$ ]]; then
                    # emit a warning
                    echo "Warning: The file name ${stripped_name} is longer than 16 characters"
                fi

                # if there is no .toml file, create one
                if [[ ! -f "${TEXTURES_PATH}/${stripped_name}.material.toml" ]]; then
                    echo "inherits = \"base.material.toml\"" > "${TEXTURES_PATH}/${stripped_name}.material.toml"
                fi

                # ensure there is a directory with the same name as the texture
                mkdir -p "${TEXTURES_PATH}/${stripped_name}"
            fi
        done
    done
}

main "$@"
exit "$?"
