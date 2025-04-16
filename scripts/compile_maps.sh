#!/bin/bash
set -euo pipefail

help_text="Usage: $0 [options]
Options:
  -q, --qbsp <path>    Path to qbsp executable
  -l, --light <path>   Path to light executable
  -h, --help         Show this help message"


# set default values
qbsp_path="qbsp"
light_path="light"

# source for how to parse arguments: https://stackoverflow.com/a/14203146
while [[ $# -gt 0 ]]; do
  case $1 in
    -q|--qbsp)
      qbsp_path="$2"
      shift # past argument
      shift # past value
      ;;
    -l|--light)
      light_path="$2"
      shift # past argument
      shift # past value
      ;;
    -h|--help)
      echo "$help_text"
      exit 0
      shift # past argument
      ;;
    -*|--*)
      echo "Unknown option $1"
      exit 1
      ;;
    *)
      echo "No positional arguments allowed"
      exit 1
      ;;
  esac
done

# check if qbsp and light exist
if ! type "$qbsp_path" > /dev/null 2>&1; then
    echo "qbsp executable not found at '$qbsp_path'"
    exit 1
fi
if ! type "$light_path" > /dev/null 2>&1; then
    echo "light executable not found at '$light_path'"
    exit 1
fi


for map in assets/maps/**/*.map; do
    echo "Compiling $map"
    $qbsp_path -bsp2 -wrbrushesonly -nosubdivide -nosoftware -path assets -notex $map ${map%.map}.bsp
    $light_path -extra4 -novanilla -lightgrid -path assets ${map%.map}.bsp
done
