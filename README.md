# C++11 vs Rust comparison

## Intro

The goal of this project is to understand programming idioms of Rust, and
roughly measure Rust performance compared to C++11. For this purpose several
well known algorithmic problems were considered. The problems were inspired and
evolved from great computer science course "Algorithms: Theory & Practice" on
Stepik paltform. Here the link to the course:
https://stepik.org/course/217/syllabus . 

## Problems

Here is short overview of the programming problems considered in the project.
More detailed description of each task could be found in the tasks' folders.

`huffman_encoding` and `huffman_decoding` are about of building optimal prefix
code for the given message and decoding it back. These problems are about to
work with priority queque (heap data structure), binary tree and hash map.
Particular realisations on Rust and C++11 relies on standard libraries'
realisation of hash map and priority queue implementing only binary tree to
solve the problem. Complexity is O(N) for building the code.

`binary_search` is quite well known and straightforward algorithm for searching
of the particular value in a sorted array. Here is no significant distinctions
in realisations. Complexity is O(logN).

`mergesort` algorithm of sorting is implemented in non-recursion way in order
to avoid stack overflow. Complexity is O(N logN).

`levenshtein` distance is usually used to quantify the difference between two
strings. The algorithm is implemented in dynamic prgramming manner. Complexity is O(N^2).

## Performance results
![Performance table](performance_table.png?raw=true "Performance table")

`(core)` means measurments of only algorithmic part of the programs (excluding
startup, input-output operations)

## Folders overview

Each problem can be found in the folder with the same name. In each folder here
are `main.cpp` and `main.rs` source codes which is solving the described
problems in C++11 and Rust. Also here is `Makefile` in each folder to compile
the programs, prepare test data and perform performance measures. The common
part of the `Makefile` can be found in `common/common.make`.

Also each problem's folder contains `generator` project on Rust which is
dedicated to generate input data for prgrams for testing purposes.

Although one can find brief overview of the problems above, in the particular
folder here is `README.md` with more formal problem descriptions and notes.

## Performance testing set up

Each problem has Make scenario with 4 rules: `main_rust`, `main_cpp`, `gen` and
`bench`. First two are about compiling the programs. Every program recieves its
input on `stdin` and write results in `stdout`. `gen` will prepare test data:
three input sets `inp_low`, `inp_mid`, `inp_hi` with increasing problem sizes.
Generally, `inp_low` contains smallest inputs, and `inp_hi` contains millions
of entities and it usually takes few seconds to solve the problem of a such size.
`bench` rule will call `common/compare_performace.sh` which will perform N (N =
10) runs of every input on evety program and everage the result. Occasianally,
here is not standard command line tool for measure the program runtime in
milisecond resolution for Mac Os X and Linux.. Therefore in `common/measure`
you will find a small rust program to meausre the runtime of another program.
To run all steps in once, one can use `run_bench.sh`. It will take up to ten
minutes to perform all compilations, generations and performance measures. 

## Compilation

The next command line is used to compile C++ program:

```
	g++ -std=c++11 -O2 -DNDEBUG -o main_cpp main.cpp
```

The next command line is used to compile Rust program:

```
	rustc -O --crate-name main_rust main.rs
```


## Requirements

One needs to run Mac Os X or Linux with installed to PATH GNU C++ compiler or Clang and Rust. By the author the presented project was tested with the next softwares:

```
> rustc --version -v                                                                                                                                                                                  9:06:07  ☁  edit1 ☂ 𝝙 ✭ 𝝙 khdmitry-pc
rustc 1.22.1 (05e2e1c41 2017-11-22)
binary: rustc
commit-hash: 05e2e1c41414e8fc73d0f267ea8dab1a3eeeaa99
commit-date: 2017-11-22
host: x86_64-unknown-linux-gnu
release: 1.22.1
LLVM version: 4.0

> g++ -v
...
gcc version 7.2.0
```

## License

Actually here is no any type of restrictions on the materials in this repository. Anyone can do anything with the source code and results. 
