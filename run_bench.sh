#!/bin/bash

tests=( huffman_encoding huffman_decoding binary_search mergesort levenshtein )
out="benchmark.txt"

# build measure tool
cd common/measure
cargo build --release &> /dev/null
cd ../..

# clean out file
if [ -f $out ]; then rm -f $out; fi;
touch $out

for t in "${tests[@]}"
do
    echo "Test $t" | tee -a $out
    cd "$t"
    echo
    echo "Compiling programs.." | tee -a $out
    make &> /dev/null
    echo "Generate test data.." | tee -a $out
    make gen &> /dev/null
    echo "Measuring performance.." | tee -a $out
    make bench 2>&1 | tee -a ../$out
    cd ..
done
