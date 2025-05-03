#!/usr/bin/env python3

import shutil
import subprocess
import sys
import os

ORIGINAL_ASSETS_DIR = "assets"
BAKED_ASSETS_DIR = "assets_baked"
TEXTURE_EXTENSIONS = [".png", ".jpg", ".jpeg"]


def main():
    verify_that_the_assets_are_in_the_working_directory()
    verify_that_all_tools_are_installed()
    create_empty_bake_directory()

    copy_truncated_textures_to_texture_root()
    copy_non_texture_files_to_bake_directory()
    convert_textures_to_ktx2()
    convert_gltf_textures_to_ktx2()

def verify_that_all_tools_are_installed():
    tools = [["kram"], ["qbsp", "--help"], ["light", "--help"]]
    for tool in tools:
        try:
            subprocess.run(
                tool, check=True, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL
            )
        except FileNotFoundError:
            print(f"{tool[0]} is not installed")
            sys.exit(1)


def verify_that_the_assets_are_in_the_working_directory():
    if not os.path.exists(ORIGINAL_ASSETS_DIR):
        print(
            f"'{ORIGINAL_ASSETS_DIR}' directory not found. Please run this script from the root of the repository."
        )
        sys.exit(1)


def create_empty_bake_directory():
    shutil.rmtree(BAKED_ASSETS_DIR, ignore_errors=True)
    os.makedirs(BAKED_ASSETS_DIR)


# This cannot be configured, it's what TrenchBroom expects
_ORIGINAL_TEXTURES_DIR = os.path.join(ORIGINAL_ASSETS_DIR, "textures")
_BAKED_TEXTURES_DIR = os.path.join(BAKED_ASSETS_DIR, "textures")


def copy_truncated_textures_to_texture_root():
    os.makedirs(_BAKED_TEXTURES_DIR, exist_ok=True)
    # we go through all the files in the textures directory
    # if we encounter the following constellation:
    # - file.png
    # - file.toml
    # - file/file_some_suffix.png
    # we know that `file.png` is a base color texture
    # and that `file/...` contains PBR textures
    # This directory structure needs to be moved directly into the BAKED_ASSETS_DIR/textures directory
    # and `file` needs to be truncated to 15 characters
    # and the PBR textures need to be renamed to match the base color texture name
    _bake_texture_recursively(_ORIGINAL_TEXTURES_DIR)


_MAX_TEXTURE_NAME_LENGTH = 15

def copy_non_texture_files_to_bake_directory():
    for entry in os.scandir(ORIGINAL_ASSETS_DIR):
        if entry.is_file():
            shutil.copy2(entry.path, os.path.join(BAKED_ASSETS_DIR, entry.name))
        elif entry.is_dir() and entry.name != "textures":
            shutil.copytree(entry.path, os.path.join(BAKED_ASSETS_DIR, entry.name))


def convert_textures_to_ktx2():
    for root, _dirs, files in os.walk(BAKED_ASSETS_DIR):
        for file in files:
            texture_name, ext_name = os.path.splitext(file)
            if ext_name == ".toml":
                # change all instances of TEXTURE_EXTENSIONS in the file to ".ktx2"
                with open(os.path.join(root, file), "r") as f:
                    content = f.read()
                for ext in TEXTURE_EXTENSIONS:
                    content = content.replace(ext, ".ktx2")
                with open(os.path.join(root, file), "w") as f:
                    f.write(content)
            elif ext_name in TEXTURE_EXTENSIONS:
                # convert the texture to ktx2
                subprocess.run(
                    [
                        "kram",
                        "encode",
                        "-input",
                        os.path.join(root, file),
                        "-output",
                        os.path.join(root, f"{texture_name}.ktx2"),
                        "-mipmin",
                        "1",
                        "-zstd",
                        "0",
                        "-format",
                        "bc7",
                        "-encoder",
                        "bcenc",
                    ],
                    check=True,
                )
                os.remove(os.path.join(root, file))

def convert_gltf_textures_to_ktx2():
    GLTF_EXTENSIONS = [".glb", ".gltf"]
    for root, _dirs, files in os.walk(BAKED_ASSETS_DIR):
        for file in files:
            if os.path.splitext(file)[1] in GLTF_EXTENSIONS:
                # search for all instances of TEXTURE_EXTENSIONS in the file
                with open(os.path.join(root, file), "r") as f:
                    content = f.read()
                for ext in TEXTURE_EXTENSIONS:
                    content = content.replace(ext, ".ktx2")
                with open(os.path.join(root, file), "w") as f:
                    f.write(content)


def _bake_texture_recursively(texture_path: str):
    with os.scandir(texture_path) as it:
        files = {entry.name: entry for entry in it}
        for file_name, file in files.items():
            texture_name, ext_name = os.path.splitext(file_name)
            truncated_texture_name = texture_name[:_MAX_TEXTURE_NAME_LENGTH]
            material_name = f"{texture_name}.toml"
            if ext_name == ".toml":
                truncated_material_name = f"{truncated_texture_name}.toml"
                shutil.copy2(
                    os.path.join(texture_path, material_name),
                    os.path.join(_BAKED_TEXTURES_DIR, truncated_material_name),
                )
                continue
            if file.is_dir():
                _bake_texture_recursively(os.path.join(texture_path, file_name))
                continue
            has_directory = texture_name in [
                file_name for file_name, file in files.items() if file.is_dir()
            ]
            has_material_file = material_name in [
                file_name for file_name, _file in files.items()
            ]
            if has_material_file:
                # we have a base color texture
                # and we need to move the directory recursively

                # copy the base color texture
                shutil.copy2(
                    file.path,
                    os.path.join(
                        _BAKED_TEXTURES_DIR, f"{truncated_texture_name}.{ext_name}"
                    ),
                )

                # copy the directory recursively
                if has_directory:
                    os.makedirs(
                        os.path.join(_BAKED_TEXTURES_DIR, truncated_texture_name),
                        exist_ok=True,
                    )
                    for pbr_file in os.scandir(
                        os.path.join(texture_path, texture_name)
                    ):
                        # the suffix after the file_name is the extension of the PBR texture
                        # e.g. foo_normal.png -> _normal
                        pbr_file_name = pbr_file.name
                        pbr_start_index = len(texture_name)
                        if len(pbr_file_name) <= pbr_start_index:
                            raise Exception(
                                f"Failed to find PBR texture for {file.path}"
                            )
                        pbr_suffix = pbr_file_name[pbr_start_index:]
                        pbr_suffix, pbr_extension = os.path.splitext(pbr_suffix)
                        shutil.copy2(
                            file.path,
                            os.path.join(
                                _BAKED_TEXTURES_DIR,
                                truncated_texture_name,
                                f"{truncated_texture_name}{pbr_suffix}.{pbr_extension}",
                            ),
                        )


if __name__ == "__main__":
    main()
