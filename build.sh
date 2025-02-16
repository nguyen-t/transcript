#!/usr/bin/env bash

git clone https://github.com/ggerganov/whisper.cpp

cd whisper.cpp

cmake -B build \
  -DCMAKE_POSITION_INDEPENDENT_CODE=ON \
  -DBUILD_SHARED_LIBS=OFF \
  -DGGML_CCACHE=OFF \
  -DGGML_OPENMP=OFF
cmake --build build --config Release 
