# jl-sys

This crate contains the raw bindings to the Julia C API, these are generated by `bindgen`. You can find the requirements for using `bindgen` in [their User Guide](https://rust-lang.github.io/rust-bindgen/requirements.html).

### Build instructions

#### Linux

The recommended way to install Julia is to download the binaries from the official website, which is distributed in an archive containing a directory called `julia-x.y.z`. This directory contains several other directories, including a `bin` directory containing the `julia` executable.

In order to ensure the `julia.h` header file can be found, either `/usr/include/julia/julia.h` must exist, or you have to set the `JULIA_DIR` environment variable to `/path/to/julia-x.y.z`. The environment variable can be used to override the default. Similarly, in order to load `libjulia.so` you must add `/path/to/julia-x.y.z/lib` to the `LD_LIBRARY_PATH` environment variable.

#### Windows

The recommended way to install Julia is to download the installer from the official website, which will install Julia in a folder called `Julia-x.y.z`. This folder contains several other folders, including a `bin` folder containing the `julia.exe` executable. You must set the `JULIA_DIR` environment variable to the `Julia-x.y.z` folder and add `Julia-x.y.z\bin` to the `PATH` environment variable. For example, if Julia is installed at `D:\Julia-x.y.z`, `JULIA_DIR` must be set to `D:\Julia-x.y.z` and `D:\Julia-x.y.z\bin` must be added to `PATH`. 

Additionally, MinGW must be installed through Cygwin. To install this and all potentially required dependencies, follow steps 1-4 of [the instructions for compiling Julia on Windows using Cygwin and MinGW](https://github.com/JuliaLang/julia/blob/v1.4.1/doc/build/windows.md#cygwin-to-mingw-cross-compiling). You must set the `CYGWIN_DIR` environment variable to the installation folder of Cygwin; this folder contains some icons, `Cygwin.bat` and folders with names like `usr` and `bin`. For example, if Cygwin is installed at `D:\cygwin64`, `CYGWIN_DIR` must be set to `D:\cygwin64`.

Julia is compatible with the GNU toolchain on Windows. If you use rustup, you can set the toolchain for a project that depends on `jl-sys` by calling the command `rustup override set stable-gnu` in the project root folder.