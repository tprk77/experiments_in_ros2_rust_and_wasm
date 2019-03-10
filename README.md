# Experiments in ROS2, Rust, and WASM #

## Overview ##

This project attempts to combine ROS2, Rust, and WASM, to demonstrate the
deployment of robotic WASM applications to a ROS2 enabled WASM runtime. This is
an _extremely_ early stage demo, with _unstable pre-alpha_ dependencies. I can't
recommend using any of this in a production environment. But nevertheless, I
think it's an interesting concept with great potential.

## Organization ##

**ROS2 WASM App:** `ros2_wasm_app_rust` and `ros2_wasm_app_cpp`

**ROS2 WASM Host:** `ros2_ws/src/ros2_rust_wasm`

The project is basically split into two parts, the app and the host. The app is
a program that is compiled to WASM. It is written against a specific simplified
ROS2 WASM API, which the host then needs to provide during the execution of the
app. The host is a native program embedding a non-web WASM runtime, and provides
the simplified ROS2 WASM API to the app. Because the host is essentially a ROS
node, it needs to be built as part of a ROS workspace.

The project also has a few dependencies that are included as Git submodules:

**ROS2 Rust Client:** `ros2_ws/src/ros2_rust`

I'm using Gérald Lelong's `lelongg/ros2_rust` Crystal branch for the ROS2 Rust
Client. The upstream ROS2 Rust Client doesn't work with Crystal and has not
accepted the corresponding pull request yet.

**Wasmer:** `extern/wasmer`

I'm using Wasmer for the non-web WASM runtime. I'm using my own fork, which
allows for combining the ROS2 imports with Emscripten imports. That is not
currently possible in the master branch. Unfortunately my fork is already out of
date and will not be merged upstream. Wasmer is a very exciting project and is
under heavy, active development.

**Emscripten:** `extern/emsdk`

Emscripten is a pretty mature project, it's basically a C and C++ compiler that
targets WASM. I'm using it to compile the C++ WASM app. Emscripten usually
targets web browsers, but can be made to work with Wasmer. See the `Makefile` in
`ros2_wasm_app_cpp` for details on how I do that.

## Setup ##

If you haven't install Rust yet, you can do that easily with [`rustup`][1]!

```text
# See also: https://rustup.rs/
curl https://sh.rustup.rs -sSf | sh
```

You will need to install ROS2. I recommend following the [instructions to install
Debian packages][2]. Here's some summary commands:

```text
sudo apt update && sudo apt install curl gnupg2 lsb-release
curl http://repo.ros2.org/repos.key | sudo apt-key add -
sudo apt update
sudo apt install ros-crystal-desktop
```

You will need to initialize and update all of the submodules in this repo. This
will get the ROS2 Rust Client, Wasmer, Emscripten, etc, setup in the correct
locations. The following command will do that:

```text
git submodule update --init --recursive
```

Once that's done, you need to [setup Emscripten][3]:

```text
cd extern/emsdk
./emsdk update
./emsdk install latest
./emsdk activate latest
```

(Note that the `emsdk` submodule will appear "dirty" after setting up
Emscripten, but you should just ignore that.)

## Building ##

You will need to build the ROS2 workspace with Colcon. Please be aware that the
`ros2_rust` build tools are not really working correctly at the moment.
Iterative builds will use stale sources, so you must do a clean build every
time. (I guess the issue has to do with CMake copying files for Cargo.)

```text
cd ros2_ws
source /opt/ros/crystal/setup.bash
rm -rf build install log && colcon build
```

You can then build the Rust app:

```text
cd ros2_wasm_app_rust
cargo build --release
```

And finally the C++ app:

```text
cd ros2_wasm_app_cpp
source ../extern/emsdk/emsdk_env.sh
make
```

## Running ##

TODO

<!-- References -->

[1]: https://rustup.rs/
[2]: https://index.ros.org/doc/ros2/Installation/Linux-Install-Debians/
[3]: https://emscripten.org/docs/getting_started/downloads.html

<!-- Local Variables: -->
<!-- fill-column: 80 -->
<!-- End: -->
