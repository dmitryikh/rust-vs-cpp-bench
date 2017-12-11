The test was launched by `run_bench.sh` script. The testing hardware is: Intel Core i7-4770, 16GB DDR3, SSD, Linux Mint 18.1 64-bit.
The output:
```
Test huffman_encoding                               

Compiling programs..
Generate test data..
Measuring performance..
../common/compare_performance.sh
Averaging performance of 10 runs..
./main_cpp:                     
  test inp_low: runtime .00173 sec, core .00028 sec
  test inp_mid: runtime .02906 sec, core .02627 sec
  test inp_hi: runtime .26562 sec, core .25239 sec 
                                                   
./main_rust:                                      
  test inp_low: runtime .00165 sec, core .00040 sec
  test inp_mid: runtime .03875 sec, core .03670 sec
  test inp_hi: runtime .41500 sec, core .40053 sec 
                                                   
Test huffman_decoding                             

Compiling programs..                                                                                                                                                                                                                                         
Generate test data.. 
Measuring performance..
../common/compare_performance.sh
Averaging performance of 10 runs..
./main_cpp:            
  test inp_low: runtime .00165 sec, core .00019 sec
  test inp_mid: runtime .02519 sec, core .01828 sec
  test inp_hi: runtime .23616 sec, core .18015 sec
                                                   
./main_rust:                                       
  test inp_low: runtime .00153 sec, core .00023 sec
  test inp_mid: runtime .02639 sec, core .02162 sec
  test inp_hi: runtime .29077 sec, core .21806 sec
                                                   
Test binary_search                                 
                                                  
Compiling programs..
Generate test data.. 
Measuring performance..
../common/compare_performance.sh                                                                                                                                                                                                                 
Averaging performance of 10 runs..                                                                                                                                                                                                               
./main_cpp:                                                                                                                                                                                                                                      
  test inp_low: runtime .00236 sec, core .00007 sec                                                                                                                                                                                              
  test inp_mid: runtime .10366 sec, core .01847 sec                                                                                                                                                                                              
  test inp_hi: runtime 1.20806 sec, core .35945 sec                                                                                                                                                                                              
                                                   
./main_rust:
  test inp_low: runtime .00179 sec, core .00008 sec
  test inp_mid: runtime .07810 sec, core .01910 sec
  test inp_hi: runtime 1.12885 sec, core .41750 sec

Test mergesort

Compiling programs..
Generate test data..
Measuring performance..
../common/compare_performance.sh
Averaging performance of 10 runs..
./main_cpp:
  test inp_low: runtime .00310 sec, core .00093 sec
  test inp_mid: runtime .19630 sec, core .12184 sec
  test inp_hi: runtime 2.19702 sec, core 1.45821 sec

./main_rust:
  test inp_low: runtime .00248 sec, core .00092 sec
  test inp_mid: runtime .17509 sec, core .12201 sec
  test inp_hi: runtime 2.00531 sec, core 1.44780 sec

Test levenshtein

Compiling programs..
Generate test data..
Measuring performance..
../common/compare_performance.sh
Averaging performance of 10 runs..
./main_cpp:
  test inp_low: runtime .00411 sec, core .00267 sec
  test inp_mid: runtime .01765 sec, core .01610 sec
  test inp_hi: runtime .07393 sec, core .07237 sec

./main_rust:
  test inp_low: runtime .00665 sec, core .00438 sec
  test inp_mid: runtime .02114 sec, core .01922 sec
  test inp_hi: runtime .08106 sec, core .07860 sec

```
