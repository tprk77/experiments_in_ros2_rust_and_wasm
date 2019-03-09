# Experiments in ROS2, Rust, and WASM #

## Overview ##

This project attempts to combine ROS2, Rust, and WASM, to demonstrate the
deployment of robotic WASM applications to a ROS2 enabled WASM runtime. This is
an _extremely_ early stage demo, with _unstable pre-alpha_ dependencies. I can't
recommend using any of this in a production environment. But nevertheless, I
think it's an interesting concept with great potential.

## Organization ##

**ROS2 WASM App:** `ros2_wasm_app_rust`

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

## Setup ##

TODO


<!-- Local Variables: -->
<!-- fill-column: 80 -->
<!-- End: -->
