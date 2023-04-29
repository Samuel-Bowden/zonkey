#!/bin/bash

# Verify the output of each program to make sure they are all doing the same thing

NAME=$1
TEST_LOX=$2

cd $NAME

echo "Testing $NAME"

echo "Running C++ (GCC)"
g++ -O3 $NAME.cpp -o $NAME 
./$NAME > gcc_output
rm $NAME 

echo "Running C++ (Clang)"
g++ -O3 $NAME.cpp -o $NAME 
./$NAME > clang_output
rm $NAME 

echo "Running Rust (rustc)"
rustc -C opt-level=3 $NAME.rs -o $NAME 
./$NAME > rust_output
rm $NAME 

echo "Running Python (CPython)"
python $NAME.py > python_output

echo "Running JavaScript (Deno)"
deno run $NAME.js > deno_output

echo "Running JavaScript (Bun)"
bun run $NAME.js > bun_output

echo "Running Zonkey"
zonkey run $NAME.zonk > zonkey_output

if [[ $TEST_LOX == "lox" ]]; then
    echo "Running Lox (CLox)"
    # Clox doesn't have a version argument
    clox $NAME.lox > lox_output
    echo "-----"

    echo "Comparing Output"
    diffuse gcc_output clang_output rust_output python_output deno_output bun_output zonkey_output lox_output
    rm lox_output
else
    echo "Comparing Output"
    diffuse gcc_output clang_output rust_output python_output deno_output bun_output zonkey_output
fi

rm gcc_output clang_output rust_output python_output deno_output bun_output zonkey_output
