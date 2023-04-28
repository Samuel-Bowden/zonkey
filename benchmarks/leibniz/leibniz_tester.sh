#!/bin/bash
CYCLES=10

echo "Running C++"
g++ --version
g++ -O3 leibniz.cpp -o leibniz
perf stat -r $CYCLES ./leibniz > /dev/null
rm leibniz
echo "-----"

echo "Running Python"
python --version
perf stat -r $CYCLES python leibniz.py > /dev/null
echo "-----"

echo "Running JavaScript (Deno)"
deno --version
perf stat -r $CYCLES deno run leibniz.js > /dev/null
echo "-----"

echo "Running JavaScript (Bun)"
bun --version
perf stat -r $CYCLES bun run leibniz.js > /dev/null
echo "-----"

echo "Running Zonkey"
zonkey --version
perf stat -r $CYCLES zonkey run leibniz.zonk > /dev/null
echo "-----"
