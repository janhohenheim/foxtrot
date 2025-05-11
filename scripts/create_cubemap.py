#!/usr/bin/env python3

import subprocess
import os
from pathlib import Path
import sys

def run_command(cmd, check=True):
    """Run a shell command and return its output."""
    try:
        result = subprocess.run(cmd, shell=True, check=check, text=True, capture_output=True)
        return result.stdout
    except subprocess.CalledProcessError as e:
        print(f"Error running command: {cmd}")
        print(f"Error: {e.stderr}")
        if check:
            sys.exit(1)
        return None

def create_cubemap(input_file):
    # Convert input path to Path object
    input_path = Path(input_file)
    file_name = input_path.stem  # Get filename without extension

    # Create cubemap from equirectangular
    cmd = f'exrenvmap -c -li -w 512 -m -z none "{input_path}" "cubemap_%.exr"'
    run_command(cmd)
    print("Created cubemap from equirectangular")

    # Process each cubemap face
    for face in Path(".").glob("cubemap_*.exr"):
        # Fix with ImageMagick
        run_command(f'magick "{face}" "{face}"')
        # Fix with OIIO
        run_command(f'oiiotool "{face}" --fixnan box3 -o "{face}"')
        print(f"Processed {face}")

    # Remove existing KTX2 file if it exists
    ktx2_file = f"{file_name}.ktx2"
    if os.path.exists(ktx2_file):
        os.remove(ktx2_file)

    # Create KTX2 file
    cmd = (
        f'ktx create --format R16G16B16A16_SFLOAT '
        f'--assign-tf linear '
        f'--cubemap '
        f'--zstd 3 '
        f'"cubemap_+X.exr" "cubemap_-X.exr" "cubemap_+Y.exr" "cubemap_-Y.exr" '
        f'"cubemap_-Z.exr" "cubemap_+Z.exr" '
        f'"{ktx2_file}"'
    )
    run_command(cmd)
    print(f"Created {ktx2_file}")

    # Get KTX2 info
    info = run_command(f'ktx info "{ktx2_file}" | grep "vkFormat"')
    print(info)

def main():
    if len(sys.argv) != 2:
        print("Usage: python create_cubemap.py <input_file.exr>")
        sys.exit(1)

    input_file = sys.argv[1]
    create_cubemap(input_file)

if __name__ == "__main__":
    main()

