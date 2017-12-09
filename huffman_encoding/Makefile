include ../common/common.make

gen:
	cd ./generator;\
	echo 10000    | cargo run > ../inp_low;\
	echo 1000000  | cargo run > ../inp_mid;\
	echo 10000000 | cargo run > ../inp_hi;\
	cd ..;\
	cat inp_low | ./main_rust > ../huffman_decoding/inp_low;\
	cat inp_mid | ./main_rust > ../huffman_decoding/inp_mid;\
	cat inp_hi  | ./main_rust > ../huffman_decoding/inp_hi;\
