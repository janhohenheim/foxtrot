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

            # Warn if the base color name (without the extension) is longer than 16 characters
            local stripped_name=$(basename "${file}" ".${extension}")
            local non_base_suffixes=('_local' '_disp' '_arm' '_nor')
            local has_suffix=false
            for suffix in "${non_base_suffixes[@]}"; do
                if [[ "$stripped_name" == *"$suffix"* ]]; then
                    has_suffix=true
                    break
                fi
            done
            if [[ "$has_suffix" == false ]]; then
                if [[ "${stripped_name}" =~ ^.{17,}$ ]]; then
                    # emit a warning
                    echo "Warning: The file name ${stripped_name} is longer than 16 characters"
                fi
            fi
        done
    done
}

main "$@"
exit "$?"
