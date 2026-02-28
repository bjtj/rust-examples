#!/usr/bin/env bash

make
export LD_LIBRARY_PATH=$(realpath ./c_lib_shared)
./rust_app/target/debug/rust_app 
