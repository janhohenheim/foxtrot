#!/usr/bin/env python3

# This script runs a bunch of optimizations on your assets to make them
# more suitable for release builds. It is executed automatically by the release workflow,
# but you can also run it locally if you want to run `bevy run --release`.
# Make sure to have installed `kram`, `qbsp`, `light`, and `klafsa` in your PATH.

import shutil
import subprocess
import sys
import os

ORIGINAL_ASSETS_DIR = "assets"
BAKED_ASSETS_DIR = "assets_baked"
TEXTURE_EXTENSIONS = [".png", ".jpg", ".jpeg"]

MODELS_SUB_DIR = "models"
NORMAL_MAP_SUFFIX = ["_normal", "_local"]
LINEAR_TEXTURE_SUFFIX = ["_metallic", "_roughness", "_ambient_occlusion", "_emissive", "_depth", "_disp"]


def main():
    verify_that_the_assets_are_in_the_working_directory()
    verify_that_all_tools_are_installed()
    create_empty_bake_directory()

    print("Readying textures for qbsp")
    copy_truncated_textures_to_texture_root()
    print("Copying non-texture files to bake directory")
    copy_non_texture_files_to_bake_directory()

    print("Compiling maps")
    # note that this needs to be done before converting the textures to ktx2
    # because the quake tools cannot read that format
    compile_maps()

    print("Converting all textures to ktx2")
    convert_textures_to_ktx2()
    print("Telling glTF files to use ktx2 textures")
    convert_gltf_textures_to_ktx2()


def verify_that_all_tools_are_installed():
    tools = [["kram"], ["qbsp", "--help"], ["light", "--help"], ["klafsa", "--help"]]
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


_texture_name_counter: int = 0
_texture_name_to_truncated_name: dict[str, str] = {}
_texture_renames: dict[str, str] = {}
_linear_textures: set[str] = set()
_normal_maps: set[str] = set()


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


def copy_non_texture_files_to_bake_directory():
    for entry in os.scandir(ORIGINAL_ASSETS_DIR):
        if entry.is_file():
            shutil.copy2(entry.path, os.path.join(BAKED_ASSETS_DIR, entry.name))
        elif entry.is_dir() and entry.name != "textures":
            shutil.copytree(entry.path, os.path.join(BAKED_ASSETS_DIR, entry.name))


def convert_textures_to_ktx2():
    for root, _dirs, files in os.walk(BAKED_ASSETS_DIR):
        if root.startswith(f"{BAKED_ASSETS_DIR}/{MODELS_SUB_DIR}"):
            continue
        for file in files:
            texture_name, ext_name = os.path.splitext(file)
            if ext_name in TEXTURE_EXTENSIONS:
                file_path = os.path.join(root, file)
                print(f"\tConverting {file_path} to ktx2")

                # kram encode -input your_image.png -output your_image.ktx2 -mipmin 1 -zstd 0 -format bc7 -encoder bcenc
                command = [
                    "kram",
                    "encode",
                    "-input",
                    file_path,
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
                ]
                if file_path in _normal_maps:
                    command.append("-normal")
                elif file_path in _linear_textures:
                    pass
                else:
                    command.append("-srgb")

                # convert the texture to ktx2
                subprocess.run(
                    command,
                    check=True,
                )
                os.remove(os.path.join(root, file))


def convert_gltf_textures_to_ktx2():
    GLTF_EXTENSIONS = [".glb", ".gltf"]
    for root, _dirs, files in os.walk(os.path.join(BAKED_ASSETS_DIR, MODELS_SUB_DIR)):
        for file in files:
            if os.path.splitext(file)[1] in GLTF_EXTENSIONS:
                print(f"\tConverting {os.path.join(root, file)} to use ktx2")
                subprocess.run(
                    [
                        "klafsa",
                        "-b",
                        "kram",
                        "--codec",
                        "bc7",
                        "--container",
                        "ktx2",
                        "gltf",
                        os.path.join(root, file),
                    ],
                    check=True,
                )
                # remove the original file
                os.remove(os.path.join(root, file))
                # rename the ktx2 file to the original file name
                file_name = os.path.splitext(file)[0]
                os.rename(
                    os.path.join(root, f"{file_name}_bc7_ktx2.gltf"),
                    os.path.join(root, file),
                )

    for root, _dirs, files in os.walk(os.path.join(BAKED_ASSETS_DIR, MODELS_SUB_DIR)):
        for file in files:
            if os.path.splitext(file)[1] in TEXTURE_EXTENSIONS:
                # remove the original file
                os.remove(os.path.join(root, file))


def compile_maps():
    for root, _dirs, files in os.walk(BAKED_ASSETS_DIR):
        if root.endswith("/autosave"):
            # remove the autosave directory
            shutil.rmtree(root)
            continue
        for file in files:
            map_name, ext_name = os.path.splitext(file)
            if ext_name == ".map":
                file_path = os.path.join(root, file)
                with open(file_path, "r") as f:
                    content = f.read()
                for old_path, new_path in _texture_renames.items():
                    old_path = f" {old_path} "
                    new_path = f" {new_path} "
                    content = content.replace(old_path, new_path)
                with open(file_path, "w") as f:
                    f.write(content)

                bsp_path = os.path.join(root, f"{map_name}.bsp")

                print(f"\tCompiling {file_path} to {bsp_path}")
                #  qbsp -bsp2 -wrbrushesonly -nosubdivide -nosoftware -path assets_baked -notex
                subprocess.run(
                    [
                        "qbsp",
                        "-bsp2",
                        "-wrbrushesonly",
                        "-nosubdivide",
                        "-nosoftware",
                        "-path",
                        BAKED_ASSETS_DIR,
                        "-notex",
                        file_path,
                        bsp_path,
                    ],
                    check=True,
                )
                print(f"\tLighting {bsp_path}")

                subprocess.run(
                    [
                        "light",
                        "-extra4",
                        "-novanilla",
                        "-lightgrid",
                        "-path",
                        BAKED_ASSETS_DIR,
                        bsp_path,
                    ],
                    check=True,
                )
                for file in os.scandir(root):
                    if file.is_file() and file.name.endswith(
                        (".log", ".prt", ".pts", ".json")
                    ):
                        os.remove(file.path)
                # Need to keep it around for the base color preload hack :/
                #os.remove(file_path)


def _bake_texture_recursively(texture_path: str):
    # dictated by the Quake 1 BSP format
    _MAX_TEXTURE_NAME_LENGTH = 15
    with os.scandir(texture_path) as it:
        files = {entry.name: entry for entry in it}
        for file_name, file in files.items():
            texture_name, ext_name = os.path.splitext(file_name)

            # if the file is a toml file and there is a $ sign in the content, we need to rename the texture
            skip_renaming = False
            if ext_name == ".toml":
                with open(file.path, "r") as f:
                    content = f.read()
                if "$" in content and not "inherits" in content:
                    if len(texture_name) > _MAX_TEXTURE_NAME_LENGTH:
                        raise Exception(
                            f"Base material name {texture_name} is too long. Max length is {_MAX_TEXTURE_NAME_LENGTH} characters."
                        )
            if len(texture_name) <= _MAX_TEXTURE_NAME_LENGTH:
                skip_renaming = True


            material_name = f"{texture_name}.toml"
            if ext_name == ".toml":
                if not skip_renaming:
                    truncated_texture_name = _trunctate_texture_name(texture_name)
                else:
                    truncated_texture_name = texture_name
                truncated_material_name = f"{truncated_texture_name}.toml"
                baked_material_path = os.path.join(
                    _BAKED_TEXTURES_DIR, truncated_material_name
                )
                shutil.copy2(
                    os.path.join(texture_path, material_name),
                    baked_material_path,
                )

                # change all instances of TEXTURE_EXTENSIONS in the file to ".ktx2"
                with open(baked_material_path, "r") as f:
                    content = f.read()
                for ext in TEXTURE_EXTENSIONS:
                    content = content.replace(ext, ".ktx2")
                with open(baked_material_path, "w") as f:
                    f.write(content)
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
                if not skip_renaming:
                    truncated_texture_name = _trunctate_texture_name(texture_name)
                else:
                    truncated_texture_name = texture_name
                # we have a base color texture
                # and we need to move the directory recursively

                # copy the base color texture
                baked_texture_path = os.path.join(
                    _BAKED_TEXTURES_DIR, f"{truncated_texture_name}{ext_name}"
                )
                shutil.copy2(
                    file.path,
                    baked_texture_path,
                )

                bsp_texture_path = file.path.removeprefix(
                    f"{_ORIGINAL_TEXTURES_DIR}/"
                ).removesuffix(ext_name)
                _texture_renames[bsp_texture_path] = truncated_texture_name
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
                        truncated_pbr_texture_path = os.path.join(
                            _BAKED_TEXTURES_DIR,
                            truncated_texture_name,
                            f"{truncated_texture_name}{pbr_suffix}{pbr_extension}",
                        )
                        shutil.copy2(
                            pbr_file.path,
                            truncated_pbr_texture_path,
                        )
                        if pbr_suffix in NORMAL_MAP_SUFFIX:
                            _normal_maps.add(truncated_pbr_texture_path)
                        elif pbr_suffix in LINEAR_TEXTURE_SUFFIX:
                            _linear_textures.add(truncated_pbr_texture_path)

def _trunctate_texture_name(texture_name: str) -> str:
    global _texture_name_counter
    if texture_name not in _texture_name_to_truncated_name:
        _texture_name_to_truncated_name[texture_name] = str(
            _texture_name_counter
        )
        _texture_name_counter += 1
    return _texture_name_to_truncated_name[texture_name]

if __name__ == "__main__":
    main()
