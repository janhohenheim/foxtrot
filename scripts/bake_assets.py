#!/usr/bin/env python3

import shutil
import subprocess
import sys
import os

ORIGINAL_ASSETS_DIR = "assets"
BAKED_ASSETS_DIR = "assets_baked"


def main():
    verify_that_the_assets_are_in_the_working_directory()
    verify_that_all_tools_are_installed()
    create_empty_bake_directory()

    bake_textures()

def verify_that_all_tools_are_installed():
    tools = [["kram"], ["qbsp", "--help"], ["light", "--help"]]
    for tool in tools:
        try:
            subprocess.run(tool, check=True, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
        except FileNotFoundError:
            print(f"{tool[0]} is not installed")
            sys.exit(1)

def verify_that_the_assets_are_in_the_working_directory():
    if not os.path.exists(ORIGINAL_ASSETS_DIR):
        print(f"'{ORIGINAL_ASSETS_DIR}' directory not found. Please run this script from the root of the repository.")
        sys.exit(1)


def create_empty_bake_directory():
    shutil.rmtree(BAKED_ASSETS_DIR, ignore_errors=True)
    os.makedirs(BAKED_ASSETS_DIR)

# This cannot be configured, it's what TrenchBroom expects
_ORIGINAL_TEXTURES_DIR = os.path.join(ORIGINAL_ASSETS_DIR, "textures")
_BAKED_TEXTURES_DIR = os.path.join(BAKED_ASSETS_DIR, "textures")

def bake_textures():
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
    bake_texture_recursively(_ORIGINAL_TEXTURES_DIR)

def bake_texture_recursively(texture_path: str):
    with os.scandir(texture_path) as it:
        files = {entry.name: entry for entry in it}
        for file_name, file in files.items():
            [texture_name, ext_name] = os.path.splitext(file_name)
            material_name = f"{texture_name}.toml"
            if ext_name == ".toml":
                shutil.copy2(os.path.join(texture_path, material_name), os.path.join(_BAKED_TEXTURES_DIR, material_name))
                continue
            if file.is_dir():
                bake_texture_recursively(os.path.join(texture_path, file_name))
                continue
            has_directory = texture_name in [file_name for file_name, file in files.items() if file.is_dir()]
            has_material_file = material_name in [file_name for file_name, _file in files.items()]
            if  has_material_file:
                # we have a base color texture
                # and we need to move the directory recursively

                # copy the base color texture
                shutil.copy2(file.path, os.path.join(_BAKED_TEXTURES_DIR, file_name))

                # copy the directory recursively
                if has_directory:
                    shutil.copytree(os.path.join(texture_path, texture_name), os.path.join(_BAKED_TEXTURES_DIR, texture_name))



def get_extension(file: os.DirEntry) -> str:
    return os.path.splitext(file.name)[1]


if __name__ == "__main__":
    main()

