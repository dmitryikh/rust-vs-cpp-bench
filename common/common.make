all: main_rust main_cpp

main_cpp: main.cpp
	g++ -std=c++11 -O2 -DNDEBUG -o main_cpp main.cpp

main_rust: main.rs
	rustc -O --crate-name main_rust main.rs

bench:
	../common/compare_performance.sh
