#!/bin/bash

# Uses https://github.com/mate-h/blender-envmap
PATH_TO_BLENDER_ENVMAP_SCRIPT=~/.local/opt/blender-envmap

# hack
cp $1 $PATH_TO_BLENDER_ENVMAP_SCRIPT/assets/ballawley_park_2k.exr

# Make sure you used 2k images!
blender -b $PATH_TO_BLENDER_ENVMAP_SCRIPT/eq2cube.blend --python $PATH_TO_BLENDER_ENVMAP_SCRIPT/env_map.py -- $1
rm -rf output/