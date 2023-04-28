#!/bin/bash
CYCLES=10

echo "Running C++"
g++ --version
g++ -O3 objects_arrays.cpp -o objects_arrays
perf stat -r $CYCLES ./objects_arrays > /dev/null
rm objects_arrays
echo "-----"

echo "Running Python"
python --version
perf stat -r $CYCLES python objects_arrays.py > /dev/null
echo "-----"

echo "Running JavaScript (Deno)"
deno --version
perf stat -r $CYCLES deno run objects_arrays.js > /dev/null
echo "-----"

echo "Running JavaScript (Bun)"
bun --version
perf stat -r $CYCLES bun run objects_arrays.js > /dev/null
echo "-----"

echo "Running Zonkey"
zonkey --version
perf stat -r $CYCLES zonkey run objects_arrays.zonk > /dev/null
echo "-----"
