## Setup

### VS Code & Rust

1. Install Visual Studio Code
2. Install the [Rust extension](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust), following all of the instructions on that page.

### CMake

1. Install [CMake](https://github.com/Kitware/CMake/releases/download/v3.18.1/cmake-3.18.1-win64-x64.msi)
2. Add the CMake binary path to the PATH environment variable (can be done during the install)

### windows-msvc Specific Setup from [vulkano-rs README.md](https://github.com/vulkano-rs/vulkano/blob/master/README.md)

1. `rustup default stable-x86_64-pc-windows-msvc`
2. Install [Build Tools for Visual Studio 2017](https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2017). If you have already been using this toolchain then its probably already installed.
3.  Install [msys2](http://www.msys2.org/), following ALL of the instructions.
4.  Then in the msys2 terminal run: `pacman --noconfirm -Syu mingw-w64-x86_64-cmake mingw-w64-x86_64-python2 mingw-w64-x86_64-ninja`
5.  Add the msys2 mingw64 binary path to the PATH environment variable.

### Python

1. Install [Python](https://www.python.org/downloads/)
2. Add the Python binary path to the PATH environment variable (can be done during the install)