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

_TEXTURES_DIR = os.path.join(ORIGINAL_ASSETS_DIR, "textures")

def bake_textures():
    # This cannot be configured, it's what TrenchBroom expects
    os.makedirs(TEXTURES_DIR, exist_ok=True)
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
    bake_texture_recursively(TEXTURES_DIR)

def bake_texture_recursively(texture_path: str):
    with os.scandir(texture_path) as it:
        files = [entry for entry in it]
        for file in files:
            name = file.name
            texture_name = os.path.splitext(name)[0]
            has_directory = texture_name in [file.name for file in files if file.is_dir()]
            has_material_file = texture_name in [file.name for file in files if get_extension(file) == ".toml"]
            if has_directory and has_material_file:
                # we have a base color texture
                # and we need to move the directory recursively



def get_extension(file: os.DirEntry) -> str:
    return os.path.splitext(file.name)[1]


if __name__ == "__main__":
    main()

