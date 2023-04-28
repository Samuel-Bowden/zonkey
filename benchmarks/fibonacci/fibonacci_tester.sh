#!/bin/bash
CYCLES=10

echo "Running C++"
g++ --version
g++ -O3 fibonacci.cpp -o fibonacci
perf stat -r $CYCLES ./fibonacci > /dev/null
rm fibonacci
echo "-----"

echo "Running Python"
python --version
perf stat -r $CYCLES python fibonacci.py > /dev/null
echo "-----"

echo "Running JavaScript (Deno)"
deno --version
perf stat -r $CYCLES deno run fibonacci.js > /dev/null
echo "-----"

echo "Running JavaScript (Bun)"
bun --version
perf stat -r $CYCLES bun run fibonacci.js > /dev/null
echo "-----"

echo "Running Zonkey"
zonkey --version
perf stat -r $CYCLES zonkey run fibonacci.zonk > /dev/null
echo "-----"
