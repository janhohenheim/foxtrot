#!/usr/bin/env python3

import subprocess
import sys

def main():
    verify_that_tools_are_installed()

def verify_that_tools_are_installed():
    tools = [["kram"], ["qbsp", "--help"], ["light", "--help"]]
    for tool in tools:
        try:
            subprocess.run(tool, check=True, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
        except FileNotFoundError:
            print(f"{tool[0]} is not installed")
            sys.exit(1)



if __name__ == "__main__":
    main()

