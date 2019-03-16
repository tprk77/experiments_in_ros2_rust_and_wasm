# Experiments in ROS2, Rust, and Wasm #

## Overview ##

This project implements the classic ROS "Hello World" publisher and subscriber,
but with a few twists. First, this project is using the newer ROS2, as opposed
to the more battle-tested ROS1. Second, it's written in Rust. Rust is a systems
programming language like C++, but it's not commonly used with ROS. Lastly, the
publisher and subscriber are compiled to WebAssembly (Wasm). They are executed
in a non-web embedding of a Wasm runtime.

This project attempts to combine ROS2, Rust, and Wasm, to demonstrate the
deployment of robotic Wasm applications to a ROS2-enabled Wasm runtime. This is
an _extremely_ early stage demo, with _unstable pre-alpha_ dependencies. I can't
recommend using this in a production environment. But nevertheless, I think it's
an interesting concept with great potential.

## Organization ##

**ROS2 Wasm App:** `ros2_wasm_app_rust` and `ros2_wasm_app_cpp`

**ROS2 Wasm Host:** `ros2_ws/src/ros2_rust_wasm`

The project is basically split into two parts, the app and the host. The app is
a program that is compiled to Wasm. It is written against a specific simplified
ROS2 Wasm API, which the host then needs to provide during the execution of the
app. The host is a native program embedding a non-web Wasm runtime, and provides
the simplified ROS2 Wasm API to the app. Because the host is essentially a ROS
node, it needs to be built as part of a ROS workspace.

The project also has a few dependencies that are included as Git submodules:

**ROS2 Rust Client:** `ros2_ws/src/ros2_rust`

I'm using Gérald Lelong's `lelongg/ros2_rust` Crystal branch for the ROS2 Rust
Client. The upstream ROS2 Rust Client doesn't work with Crystal and has not
accepted the corresponding pull request yet.

**Wasmer:** `extern/wasmer`

I'm using Wasmer for the non-web Wasm runtime. I'm using my own fork, which
allows for combining the ROS2 imports with Emscripten imports. That is not
currently possible in the master branch. Unfortunately my fork is already out of
date and will not be merged upstream. Wasmer is a very exciting project and is
under heavy, active development.

**Emscripten:** `extern/emsdk`

Emscripten is a pretty mature project, it's basically a C and C++ compiler that
targets Wasm. I'm using it to compile the C++ Wasm app. Emscripten usually
targets web browsers, but can be made to work with Wasmer. See the `Makefile` in
`ros2_wasm_app_cpp` for details on how I do that.

## Setup ##

If you haven't install Rust yet, you can do that easily with [`rustup`][1]!

```text
# See also: https://rustup.rs/
curl https://sh.rustup.rs -sSf | sh
```

You may also need to enable the `wasm32-unknown-unknown` target.

```text
rustup target add wasm32-unknown-unknown
```

You will also need to install Clang, `ros2_rust` requires it:

```text
sudo apt-get update
sudo apt-get install clang-6.0
```

You will need to install ROS2. I recommend following the [instructions to
install Debian packages][2]. Here's some summary commands:

```text
sudo sh -c 'echo "deb [arch=amd64,arm64] http://packages.ros.org/ros2/ubuntu `lsb_release -cs` main" \
    > /etc/apt/sources.list.d/ros2-latest.list'
curl http://repo.ros2.org/repos.key | sudo apt-key add -
sudo apt-get update
sudo apt-get install ros-crystal-desktop python3-colcon-common-extensions
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

You will need to build the ROS2 workspace with Colcon.

```text
cd ros2_ws
source /opt/ros/crystal/setup.bash
colcon build --packages-up-to ros2_rust_wasm
```

You can then build the Rust apps:

```text
cd ros2_wasm_app_rust
cargo build --release
```

```text
cd ros2_wasm_app_rust_subscriber
cargo build --release
```

And finally the C++ app:

```text
cd ros2_wasm_app_cpp
source ../extern/emsdk/emsdk_env.sh
make
```

## Running ##

First start the subscriber:

```text
cd ros2_ws
source install/setup.bash

# Run the Rust subscriber in Wasm!
ros2 run ros2_rust_wasm ros2_rust_wasm -w \
    ../ros2_wasm_app_rust_subscriber/target/wasm32*/release/ros2_wasm_app_rust_subscriber.wasm &
```

Then run either of the publisher apps:

```text
cd ros2_ws
source install/setup.bash

# Run the Rust publisher in Wasm!
ros2 run ros2_rust_wasm ros2_rust_wasm -w \
    ../ros2_wasm_app_rust/target/wasm32*/release/ros2_wasm_app_rust.wasm
```

```text
cd ros2_ws
source install/setup.bash

# Run the C++ publisher in Wasm!
ros2 run ros2_rust_wasm ros2_rust_wasm -w \
    ../ros2_wasm_app_cpp/build/ros2_wasm_app_cpp.wasm
```

You should get output similar to this:

```text
[INFO] This is NOT an Emscripten module!
[TRACE] rn_get_default_context
[TRACE] rn_create_node (0)
[TRACE] rn_create_subscription (0)
[TRACE] rn_node_spin
...
[INFO] This is NOT an Emscripten module!
[TRACE] rn_get_default_context
[TRACE] rn_create_node (0)
[TRACE] rn_create_publisher (0)
[TRACE] rn_std_msg_string_default
[TRACE] rn_std_msg_string_set_data (0)
[TRACE] rn_publish (0, 0)
[TRACE] rn_thread_sleep
[TRACE] rn_std_msg_string_get_data_len (0)
[TRACE] rn_std_msg_string_get_data (0)
[TRACE] rn_log
[LOG] Hello Rust ROS! 0
...
```

## Summary ##

Wasm allows for language agnostic apps. There are two apps, written in two
different languages, written against the same API and targeting the same host.
The apps are portable. They only need to be compiled once; they can run wherever
the host can. Additionally, the apps are sandboxed. We can choose to expose only
a very specific API, block syscalls, etc. There is also the potential to recover
gracefully from app crashes.

<!-- References -->

[1]: https://rustup.rs/
[2]: https://index.ros.org/doc/ros2/Installation/Linux-Install-Debians/
[3]: https://emscripten.org/docs/getting_started/downloads.html

<!-- Local Variables: -->
<!-- fill-column: 80 -->
<!-- End: -->
