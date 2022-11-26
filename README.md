# cdtk-cpu-pixel-shader
cdtk-cpu-pixel-shader is a rust program that render to the window's buffer directly with no GPU acceleration. by using all the logical cores of the CPU to render in an interface like a GPU pixel shader for infinite parallelization opportunies!

Tutorial: https://www.youtube.com/watch?v=Rr93h-exnPA

Dependencies:

Windowing library (winit): https://github.com/rust-windowing/winit

Window buffer library (softbuffer): https://github.com/john01dav/softbuffer

Multithreading library (rayon): https://github.com/rayon-rs/rayon
