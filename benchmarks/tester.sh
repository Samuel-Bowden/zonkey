#!/bin/bash

# Test the performance of each implementation

NAME=$1
CYCLES=$2
TEST_LOX=$3

echo "Testing the performance of $NAME in each language. Output available in results folder."

mkdir -p results/$NAME

cd $NAME

echo "Running C++ (GCC)"
g++ --version
g++ -O3 $NAME.cpp -o $NAME
perf stat -r $CYCLES ./$NAME 2> ../results/$NAME/gcc.txt
rm $NAME 
echo "-----"

echo "Running C++ (Clang)"
clang --version
clang++ -O3 $NAME.cpp -o $NAME
perf stat -r $CYCLES ./$NAME 2> ../results/$NAME/clang.txt
rm $NAME
echo "-----"

echo "Running Rust (rustc)"
rustc --version
rustc -C opt-level=3 $NAME.rs -o $NAME 
perf stat -r $CYCLES ./$NAME 2> ../results/$NAME/rustc.txt
rm $NAME
echo "-----"

echo "Running Python (CPython)"
python --version
perf stat -r $CYCLES python $NAME.py 2> ../results/$NAME/python.txt
echo "-----"

echo "Running JavaScript (Deno)"
deno --version
perf stat -r $CYCLES deno run $NAME.js 2> ../results/$NAME/deno.txt
echo "-----"

echo "Running JavaScript (Bun)"
bun --version
perf stat -r $CYCLES bun run $NAME.js 2> ../results/$NAME/bun.txt
echo "-----"

echo "Running Zonkey"
zonkey --version
perf stat -r $CYCLES zonkey run $NAME.zonk 2> ../results/$NAME/zonkey.txt
echo "-----"

if [[ $TEST_LOX == "lox" ]]; then
    echo "Running Lox (CLox)"
    # Clox doesn't have a version argument
    perf stat -r $CYCLES clox $NAME.lox 2> ../results/$NAME/lox.txt
    echo "-----"
fi
