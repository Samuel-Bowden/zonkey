#!/bin/bash
CYCLES=100

echo "Running C++"
g++ --version
g++ -O3 hello_world.cpp -o hello_world
perf stat -r $CYCLES ./hello_world > /dev/null
rm hello_world
echo "-----"

echo "Running Python"
python --version
perf stat -r $CYCLES python hello_world.py > /dev/null
echo "-----"

echo "Running JavaScript (Deno)"
deno --version
perf stat -r $CYCLES deno run hello_world.js > /dev/null
echo "-----"

echo "Running JavaScript (Bun)"
bun --version
perf stat -r $CYCLES bun run hello_world.js > /dev/null
echo "-----"

echo "Running Zonkey"
zonkey --version
perf stat -r $CYCLES zonkey run hello_world.zonk > /dev/null
echo "-----"
