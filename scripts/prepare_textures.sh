#!/usr/bin/env bash

set -euo pipefail
shopt -s globstar nullglob

# Convert only files in this directory
readonly TEXTURES_PATH='assets/textures'
# Convert only these file extensions
readonly FILE_EXTENSIONS=('jpg' 'jpeg' 'png' 'tga' 'dds')
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
        done
    done

    for file in "${TEXTURES_PATH}"/**/*.png; do
        # if the file ends in _local.png, rename it to _normal.png
        if [[ "${file}" == *"_local.png" ]]; then
            mv "${file}" "${file%_local.png}_normal.png"
        fi
    done

    for file in "${TEXTURES_PATH}"/**/*.png; do
        local stripped_name=$(basename "${file}" ".png")
        local non_base_suffixes=('_local' '_disp' '_arm' '_nor')
        local has_suffix=false
        for suffix in "${non_base_suffixes[@]}"; do
            if [[ "$stripped_name" == *"$suffix"* ]]; then
                has_suffix=true
                break
            fi
        done


        # ensure there is a directory with the same name as the texture
        if [[ "$has_suffix" == false ]]; then
            mkdir -p "${file%.png}"
        fi


        if [[ "$has_suffix" == true ]]; then
            if [[ ! -f "${file%.png}.ktx2" ]]; then
                kram encode -input "${file}" -output "${file%.png}.ktx2" -mipmin 1 -zstd 0 -format bc7 -encoder bcenc
                rm "${file}"
            fi
        else
            if [[ ! -f "${file%.png}/${stripped_name}.ktx2" ]]; then
                kram encode -input "${file}" -output "${file%.png}/${stripped_name}.ktx2" -mipmin 1 -zstd 0 -format bc7 -encoder bcenc
            fi
        fi 

        if [[ "$has_suffix" == false ]]; then
                # if there is no .toml file, create one
            local material_file="${file%.png}.toml"
            if [[ ! -f "${material_file}" ]]; then
                echo "inherits = \"/textures/base.toml\"" > "${material_file}"
            fi

        fi
    done
}

main "$@"
exit "$?"
