all: build 

build: 
	mkdir -p result
	cargo build --release
	cp ./target/release/raytracing result/
run:
	./result/raytracing
clean:
	cargo clean
.PHONY: all build run clean
