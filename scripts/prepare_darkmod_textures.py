#!/usr/bin/env python3

# This is a little helper script that converts the textures used by The Dark Mod
# to a more Bevy + TrenchBroom friendly format.

import os
import subprocess
import sys

ASSETS_DIR = "assets"
TEXTURES_SUBDIR = "textures"
# Convert these extensions to PNG
TEXTURE_EXTENSIONS = [".jpg", ".jpeg", ".dds", ".tga"]
NORMAL_MAP_SUFFIX = ["_normal", "_local"]
LINEAR_TEXTURE_SUFFIX = ["_metallic", "_roughness", "_ao", "_emissive", "_depth", "_disp"]
PHONG_SPECULAR_SUFFIX = ["_s."]


def main():
    verify_that_the_assets_are_in_the_working_directory()
    verify_that_all_tools_are_installed()
    process_textures()


def verify_that_all_tools_are_installed():
    tools = [["magick", "--version"]]
    for tool in tools:
        try:
            subprocess.run(
                tool, check=True, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL
            )
        except FileNotFoundError:
            print(f"{tool[0]} is not installed")
            sys.exit(1)


def verify_that_the_assets_are_in_the_working_directory():
    if not os.path.exists(ASSETS_DIR):
        print(
            f"'{ASSETS_DIR}' directory not found. Please run this script from the root of the repository."
        )
        sys.exit(1)


def process_textures():
    for root, dirs, files in os.walk(ASSETS_DIR):
        for file in files:
            file_name, file_ext = os.path.splitext(file)
            file_path = os.path.join(root, file)
            if file_ext in TEXTURE_EXTENSIONS:
                print(f"Processing {file_path}")
                new_file_path = os.path.join(root, f"{file_name}.png")

                if any(suffix in file_name for suffix in NORMAL_MAP_SUFFIX):
                    if f"/{TEXTURES_SUBDIR}/" in file_path:
                        name_without_suffix = "_".join(file_name.split("_")[:-1])
                        os.makedirs(
                            os.path.join(root, name_without_suffix), exist_ok=True
                        )
                        normal_map_path = os.path.join(
                            root,
                            name_without_suffix,
                            f"{name_without_suffix}_normal.png",
                        )
                    else:
                        normal_map_path = new_file_path
                    subprocess.run(
                        [
                            "magick",
                            file_path,
                            "-set",
                            "colorspace",
                            "RGB",
                            "-define",
                            "png:color-type=2",
                            "-depth",
                            "8",
                            "-channel",
                            "G",
                            "-negate",
                            normal_map_path,
                        ]
                    )
                elif any(suffix in file_name for suffix in LINEAR_TEXTURE_SUFFIX):
                    subprocess.run(
                        [
                            "magick",
                            file_path,
                            "-set",
                            "colorspace",
                            "RGB",
                            "-define",
                            "png:color-type=2",
                            "-depth",
                            "8",
                            "-alpha",
                            "on",
                            new_file_path,
                        ]
                    )
                elif any(suffix in file for suffix in PHONG_SPECULAR_SUFFIX):
                    if f"/{TEXTURES_SUBDIR}/" in file_path:
                        name_without_suffix = "_".join(file_name.split("_")[:-1])
                        os.makedirs(
                            os.path.join(root, name_without_suffix), exist_ok=True
                        )
                        roughness_path = os.path.join(
                            root,
                            name_without_suffix,
                            f"{name_without_suffix}_roughness.png",
                        )
                    else:
                        roughness_path = new_file_path
                    subprocess.run(
                        [
                            "magick",
                            file_path,
                            "-channel",
                            "RGB",
                            "-negate",
                            roughness_path,
                        ]
                    )

                else:
                    # strip _d from the new file name
                    new_file_path = new_file_path.replace("_d.", ".")
                    subprocess.run(
                        [
                            "magick",
                            file_path,
                            "-define",
                            "png:color-type=2",
                            "-depth",
                            "8",
                            "-alpha",
                            "on",
                            new_file_path,
                        ]
                    )
                    # write material.toml
                    if f"/{TEXTURES_SUBDIR}/" in file_path:
                        new_file_name = os.path.splitext(file.replace("_d.", "."))[0]
                        material_path = os.path.join(root, f"{new_file_name}.toml")
                        with open(material_path, "w") as f:
                            f.write(f'inherits = "/textures/base.toml"\n')

                os.remove(file_path)


if __name__ == "__main__":
    main()
