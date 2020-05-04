#!/bin/sh

function compile {
    glslc -O $1 -o $1.spv
}

compile src/bin/triangle.vert
compile src/bin/triangle.frag
compile src/bin/compute.comp
