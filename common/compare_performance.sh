#!/bin/bash
# Run `commands` with `tests` inputs `N` times each, and average the runtime of
# each command for each test Use `timetool` to measure the time elapsed

set -e
# number of runs to average the performance
N=10

commands=( ./main_cpp ./main_rust )
tests=( inp_low inp_mid inp_hi )
timetool=../common/measure/target/release/measure

echo "Averaging performance of $N runs.."
for command in "${commands[@]}"
do
    echo "$command:"
    for test in "${tests[@]}"
    do
        runtime_total=0.0
        for try in $(seq 1 $N)
        do
            runtime=$( $timetool $command $test )
            runtime_total=`echo "$runtime + $runtime_total" | bc`
        done
        average_runtime=`echo "scale=5; $runtime_total / $N" | bc`
        echo "  test $test: $average_runtime sec"
    done
    echo ""
done
