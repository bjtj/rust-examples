all: ./c_lib/libhello.a ./c_lib_shared/libhello_shared.so ./rust_app/target/debug/rust_app

./c_lib/libhello.a:
	cd c_lib && make

./c_lib_shared/libhello_shared.so:
	cd c_lib_shared && make

./rust_app/target/debug/rust_app:
	cd rust_app && cargo build

clean:
	cd c_lib && make clean
	cd c_lib_shared && make clean
	cd rust_app && cargo clean
