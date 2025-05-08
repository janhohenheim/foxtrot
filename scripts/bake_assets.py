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

    print("Copying all files to bake directory")
    copy_all_files_to_bake_directory()

    print("Compiling maps")

    print("Converting all textures to ktx2")
    convert_textures_to_ktx2()

    print("Pointing material files to ktx2 textures")
    point_material_files_to_ktx2()

    print("Telling glTF files to use ktx2 textures")
    convert_gltf_textures_to_ktx2()


def verify_that_all_tools_are_installed():
    tools = [["kram"], ["magick", "--help"]]
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
_BAKED_TEXTURES_DIR = os.path.join(BAKED_ASSETS_DIR, "textures")


def copy_non_texture_files_to_bake_directory():
    for entry in os.scandir(ORIGINAL_ASSETS_DIR):
        if entry.is_file():
            shutil.copy2(entry.path, os.path.join(BAKED_ASSETS_DIR, entry.name))
        elif entry.is_dir() and entry.name != "textures":
            shutil.copytree(entry.path, os.path.join(BAKED_ASSETS_DIR, entry.name))

def copy_all_files_to_bake_directory():
    os.rmdir(BAKED_ASSETS_DIR)
    shutil.copytree(ORIGINAL_ASSETS_DIR, BAKED_ASSETS_DIR, dirs_exist_ok=True)

def convert_textures_to_ktx2():
    for root, _dirs, files in os.walk(BAKED_ASSETS_DIR):
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
                    "-encoder",
                    "bcenc",
                ]
                base_color = False
                if any(suffix in file for suffix in NORMAL_MAP_SUFFIX):
                    command.extend(["-normal", "-format", "bc5"])
                elif any(suffix in file for suffix in LINEAR_TEXTURE_SUFFIX):
                    command.extend(["-format", "bc5"])
                else:
                    command.extend(["-srgb", "-format", "bc7"])
                    base_color = True

                # convert the texture to ktx2
                subprocess.run(
                    command,
                    check=True,
                )
                if base_color:
                    magick_command = [
                        "magick",
                        file_path,
                        "-quality",
                        "1",
                        os.path.join(root, f"{texture_name}.jpg"),
                    ]
                    subprocess.run(
                        magick_command,
                        check=True,
                    )
                os.remove(file_path)

def point_material_files_to_ktx2():
    for root, _dirs, files in os.walk(_BAKED_TEXTURES_DIR):
        for file in files:
            if file.endswith(".toml"):
                with open(os.path.join(root, file), "r") as f:
                    content = f.read()
                for ext in TEXTURE_EXTENSIONS:
                    content = content.replace(ext, ".ktx2")
                with open(os.path.join(root, file), "w") as f:
                    f.write(content)

def convert_gltf_textures_to_ktx2():
    GLTF_EXTENSIONS = [".glb", ".gltf"]
    for root, _dirs, files in os.walk(os.path.join(BAKED_ASSETS_DIR, MODELS_SUB_DIR)):
        for file in files:
            if os.path.splitext(file)[1] in GLTF_EXTENSIONS:
                with open(os.path.join(root, file), "r") as f:
                    content = f.read()
                for ext in TEXTURE_EXTENSIONS:
                    content = content.replace(ext, ".ktx2")
                with open(os.path.join(root, file), "w") as f:
                    f.write(content)



if __name__ == "__main__":
    main()
