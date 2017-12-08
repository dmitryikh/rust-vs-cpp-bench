include ../common/common.make

gen:
	cd ./generator;\
	echo "10000 1000 0.5"       | cargo run > ../inp_low;\
	echo "1000000 100000 0.5"   | cargo run > ../inp_mid;\
	echo "10000000 1000000 0.5" | cargo run > ../inp_hi;\
	cd ..
