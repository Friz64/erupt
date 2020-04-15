#!/bin/sh

function compile {
    glslc -O $1 -o $1.spv
}

compile src/triangle.vert
compile src/triangle.frag
