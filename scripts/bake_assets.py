#!/usr/bin/env python3

# This script runs a bunch of optimizations on your assets to make them
# more suitable for release builds. It is executed automatically by the release workflow,
# but you can also run it locally if you want to run `bevy run --release`.
# Make sure to have installed `kram`, and `magick` in your PATH.

import shutil
import subprocess
import sys
import os
import math
import concurrent.futures
import multiprocessing

ORIGINAL_ASSETS_DIR = "assets"
BAKED_ASSETS_DIR = "assets_baked"
TEXTURE_EXTENSIONS = [".png", ".jpg", ".jpeg"]

MODELS_SUB_DIR = "models"
TEXTURE_DIRS = ["models", "particles", "textures"]

NORMAL_MAP_SUFFIX = ["_normal", "_local"]
LINEAR_TEXTURE_SUFFIX = [
    "_metallic",
    "_roughness",
    "_ambient_occlusion",
    "_emissive",
    "_depth",
    "_disp",
]


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
    point_gltf_textures_to_ktx2()


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
            _ = shutil.copy2(entry.path, os.path.join(BAKED_ASSETS_DIR, entry.name))
        elif entry.is_dir() and entry.name != "textures":
            _ = shutil.copytree(entry.path, os.path.join(BAKED_ASSETS_DIR, entry.name))


def copy_all_files_to_bake_directory():
    os.rmdir(BAKED_ASSETS_DIR)
    shutil.copytree(ORIGINAL_ASSETS_DIR, BAKED_ASSETS_DIR, dirs_exist_ok=True)


def convert_textures_to_ktx2():
    files_to_process = []
    for texture_dir in TEXTURE_DIRS:
        search_path = os.path.join(BAKED_ASSETS_DIR, texture_dir)

        for root, _dirs, files in os.walk(search_path):
            for file in files:
                _, ext_name = os.path.splitext(file)
                if ext_name in TEXTURE_EXTENSIONS:
                    files_to_process.append(os.path.join(root, file))

    max_workers = max(1, multiprocessing.cpu_count() - 1)
    print(f"Processing {len(files_to_process)} textures with {max_workers} workers...")

    with concurrent.futures.ProcessPoolExecutor(max_workers=max_workers) as executor:
        list(executor.map(process_texture, files_to_process))


def process_texture(file_path):
    """
    Worker function to process a single texture file.
    """
    texture_name, _ = os.path.splitext(os.path.basename(file_path))
    root = os.path.dirname(file_path)
    output_path = os.path.join(root, f"{texture_name}.ktx2")

    resize_to_pot(file_path, file_path)

    # kram encode -input your_image.png -output your_image.ktx2 -mipmin 1 -zstd 0 -format bc7 -encoder bcenc
    command = [
        "kram",
        "encode",
        "-input",
        file_path,
        "-output",
        output_path,
        "-mipmin",
        "1",
        "-zstd",
        "0",
        "-encoder",
        "bcenc",
    ]

    base_color = False
    filename = os.path.basename(file_path)

    if any(suffix in filename for suffix in NORMAL_MAP_SUFFIX):
        command.extend(["-normal", "-format", "bc5"])
    elif any(suffix in filename for suffix in LINEAR_TEXTURE_SUFFIX):
        command.extend(["-format", "bc5"])
    else:
        command.extend(["-srgb", "-format", "bc7"])
        base_color = True

    # convert the texture to ktx2
    subprocess.run(command, check=True, capture_output=True)

    if base_color:
        jpg_output = os.path.join(root, f"{texture_name}.jpg")
        magick_command = [
            "magick",
            file_path,
            "-quality",
             "85",
            jpg_output,
        ]
        subprocess.run(magick_command, check=True, capture_output=True)

    os.remove(file_path)


def point_material_files_to_ktx2():
    for root, _dirs, files in os.walk(_BAKED_TEXTURES_DIR):
        for file in files:
            if file.endswith(".toml"):
                path = os.path.join(root, file)
                with open(path, "r") as f:
                    content = f.read()
                for ext in TEXTURE_EXTENSIONS:
                    content = content.replace(ext, ".ktx2")
                with open(path, "w") as f:
                    f.write(content)


def point_gltf_textures_to_ktx2():
    GLTF_EXTENSIONS = [".glb", ".gltf"]
    models_path = os.path.join(BAKED_ASSETS_DIR, MODELS_SUB_DIR)

    for root, _dirs, files in os.walk(models_path):
        for file in files:
            if os.path.splitext(file)[1] in GLTF_EXTENSIONS:
                path = os.path.join(root, file)
                with open(path, "r") as f:
                    content = f.read()
                for ext in TEXTURE_EXTENSIONS:
                    content = content.replace(ext, ".ktx2")
                with open(path, "w") as f:
                    f.write(content)


def resize_to_pot(input_path: str, output_path: str):
    # First, get image dimensions using `identify`
    result = subprocess.run(
        ["magick", "identify", "-format", "%w %h", input_path],
        capture_output=True,
        text=True,
        check=True,
    )
    width, height = map(int, result.stdout.split())
    if not is_power_of_two(width) or not is_power_of_two(height):
        # Compute next POT dimensions
        new_w = next_power_of_two(width)
        new_h = next_power_of_two(height)

        print(
            f"Image {input_path} has dimensions {width}x{height}, which are not power of two. Converting to {new_w}x{new_h}."
        )  # Use ImageMagick to resize canvas to POT size
        subprocess.run(
            [
                "magick",
                input_path,
                "-background",
                "transparent",  # pad with transparency
                "-gravity",
                "center",  # original image goes center
                "-extent",
                f"{new_w}x{new_h}",
                output_path,
            ],
            check=True,
            capture_output=True
        )


def next_power_of_two(x: int) -> int:
    return 1 if x == 0 else 2 ** math.ceil(math.log2(x))


def is_power_of_two(n: int) -> bool:
    return n > 0 and (n & (n - 1)) == 0


if __name__ == "__main__":
    main()
