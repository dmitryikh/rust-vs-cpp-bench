#!/bin/bash
# Run `commands` with `tests` inputs `N` times each, and average the runtime of
# each command for each test Use `timetool` to measure the time elapsed

set -e
# number of runs to average the performance
N=10

commands=( ./main_cpp ./main_rust )
tests=( inp_low inp_mid inp_hi )
timetool=../common/measure/target/release/measure
tmp_file="./tmp.file"

echo "Averaging performance of $N runs.."
for command in "${commands[@]}"
do
    echo "$command:"
    for test in "${tests[@]}"
    do
        runtime_total=0.0
        core_total=0.0
        for try in $(seq 1 $N)
        do
            runtime=$( $timetool $command $test 2> ${tmp_file})
            runtime_total=`echo "$runtime + $runtime_total" | bc`
            core_total=`echo "$( cat ${tmp_file} ) + $core_total" | bc`
        done
        average_runtime=`echo "scale=5; $runtime_total / $N" | bc`
        average_core=`echo "scale=5; $core_total / $N" | bc`
        echo "  test $test: runtime $average_runtime sec, core ${average_core} sec"
    done
    echo ""
done
