#!/bin/bash
set -e

if [ -z "$1" ]; then
    qbsp_path="qbsp"
else
    # Use passed path to qbsp. Use this in the TrenchBroom config.
    qbsp_path="$1"
fi

for map in assets/maps/**/*.map; do
    echo "Compiling $map"
    $qbsp_path -bsp2 -wrbrushesonly -nosubdivide -nosoftware -path assets -notex $map ${map%.map}.bsp
done
