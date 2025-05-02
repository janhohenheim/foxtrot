#!/usr/bin/env python3

import subprocess
import sys
import os

ORIGINAL_ASSETS_DIR = "assets"
BAKED_ASSETS_DIR = "assets_baked"

def main():
    verify_that_the_assets_are_in_the_working_directory()
    verify_that_all_tools_are_installed()

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



if __name__ == "__main__":
    main()

