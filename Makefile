PPM_VIEWER=qview

all: build 

build: 
	mkdir -p result
	cargo build --release
	cp ./target/release/raytracing result/
run:
	./result/raytracing
view:
	$(PPM_VIEWER) ./result/image.ppm
clean:
	cargo clean
.PHONY: all build run clean
