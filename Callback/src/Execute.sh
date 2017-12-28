#!/bin/sh

#1. Add the crate-type to Cargo.toml and build. Copy library here,
cp ../target/debug/deps/librustlib.so .

#2. Compile wrapper library
g++ -fpic -c wrapper.cpp -o libwrapper.o
g++ -shared -o libwrapper.so libwrapper.o
cp libwrapper.so ../target/debug/deps/


#3. Compile
g++ -ggdb test_rust.c -L. -lrustlib -lwrapper -o test

#4. Link at run time

LD_LIBRARY_PATH=. ./test

