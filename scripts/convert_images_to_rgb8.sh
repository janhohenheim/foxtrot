#!/bin/bash
set -euo pipefail

for file in $(find . -name "*.png"); do
    magick $file -depth 8 $file
done

