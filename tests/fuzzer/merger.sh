#!/bin/bash

# Merges directories of this project containing Zonkey scripts into a single directory for seeding
mkdir merged_input
find ../../ -type f -name "*.zonk" -exec cp {} merged_input \;
