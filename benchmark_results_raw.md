The test was launched by `run_bench.sh` script. The testing hardware is: Intel Core i7-4770, 16GB DDR3, SSD, Linux Mint 18.1 64-bit.
The output:
```
Test 4_2_5

Compiling programs..
Generate test data..
Measuring performance..
../common/compare_performance.sh
Averaging performance of 10 runs..
./main_cpp:
  test inp_low: .00168 sec
  test inp_mid: .02829 sec
  test inp_hi: .27003 sec

./main_rust:
  test inp_low: .00169 sec
  test inp_mid: .04197 sec
  test inp_hi: .43336 sec

Test 4_2_6

Compiling programs..
Generate test data..
Measuring performance..
../common/compare_performance.sh
Averaging performance of 10 runs..
./main_cpp:
  test inp_low: .00185 sec
  test inp_mid: .04698 sec
  test inp_hi: .43825 sec

./main_rust:
  test inp_low: .00152 sec
  test inp_mid: .02622 sec
  test inp_hi: .24990 sec

Test 6_1_4

Compiling programs..
Generate test data..
Measuring performance..
../common/compare_performance.sh
Averaging performance of 10 runs..
./main_cpp:
  test inp_low: .00288 sec
  test inp_mid: .15820 sec
  test inp_hi: 1.85318 sec

./main_rust:
  test inp_low: .00223 sec
  test inp_mid: .12787 sec
  test inp_hi: 1.55638 sec

Test 6_4_5

Compiling programs..
Generate test data..
Measuring performance..
../common/compare_performance.sh
Averaging performance of 10 runs..
./main_cpp:
  test inp_low: .00312 sec
  test inp_mid: .19478 sec
  test inp_hi: 2.10201 sec

./main_rust:
  test inp_low: .00254 sec
  test inp_mid: .15996 sec
  test inp_hi: 1.83357 sec

Test 8_3_8

Compiling programs..
Generate test data..
Measuring performance..
../common/compare_performance.sh
Averaging performance of 10 runs..
./main_cpp:
  test inp_low: .00405 sec
  test inp_mid: .01352 sec
  test inp_hi: .04957 sec

./main_rust:
  test inp_low: .00410 sec
  test inp_mid: .01416 sec
  test inp_hi: .05289 sec
```
