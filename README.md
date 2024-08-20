# Ray Tracing

This is an implementation of ray tracing following the [Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html) 
blog post. The original explains everything in `C++`, but we will attempt to follow it in `Rust`.

## Requirements

To run this project you will need to have installed:

- [Rust ecosystem](https://www.rust-lang.org/tools/install).
- [GNU Make](https://www.gnu.org/software/make/)
- An image viewer capable of opening `*.ppm` files, we use [qView](https://interversehq.com/qview/)

## Compilation and Running

To compile the program and run the resulting binary, a `Makefile` is provided.

By running:
```{=sh}
make
```
The program is compiled, and copied into `./result/`. The resulting binary can be executed by running:
```{=sh}
make run
```
And all auxiliary files generated during compilation can be cleaned with:
```{=sh}
make clean
```
