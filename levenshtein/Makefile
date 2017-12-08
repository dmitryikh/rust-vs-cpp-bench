include ../common/common.make

gen:
	cd ./generator;\
	echo "1200 500"  | cargo run > ../inp_low;\
	echo "2500 1200" | cargo run > ../inp_mid;\
	echo "5000 2500" | cargo run > ../inp_hi;\
	cd ..
