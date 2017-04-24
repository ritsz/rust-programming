#!/bin/sh

#1. Add the crate-type to Cargo.toml and build

#2. Compile
gcc test_rust.c -L. -lrustlib -o test

#3. Link at run time

LD_LIBRARY_PATH=. ./test

