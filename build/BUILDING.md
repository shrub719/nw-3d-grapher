# NumWorks 3D Grapher - Building


> **Note:** on Windows, use WSL.

## Building the app

### Setup build environment

1. Install [Node.js](https://nodejs.org/en/download)

1. Install [Nwlink](https://www.npmjs.com/package/nwlink/v/0.0.12):

   ```sh
   npm install -g nwlink
   ```

1. Install [Rust](https://rust-lang.org/tools/install/)

1. Install [Just](https://just.systems/):

   ```sh
   cargo install just
   ```

### NumWorks calculator

Building for the NumWorks calculator requires adding its Rust target:
```sh
rustup target add thumbv7em-none-eabihf
```

To build the app, run:
```sh
just build
```
This creates a binary (`.nwa`) file at `/target/thumbv7em-none-eabihf/release/nw_3d_grapher`.

To load the app to the calculator, run:
```sh
just load
```

### Simulator

To build the app for the simulator, run:
```sh
just nwb-build
```
This creates a binary (`.nwb`) file at `/target/[your Rust host]/release/libnw_3d_grapher_sim`, with a file extension according to your operating system.

### PBJ files

To import 3D models to your NumWorks calculator or simulator, use [3Dino](https://github.com/shrub719/nw-3dino) instead.


## Building the simulator

### Setup build environment

1. Install the [Epsilon SDK](https://www.numworks.com/engineering/software/build/)

1. Install [Python 3.10](https://www.python.org/downloads/release/python-3100/)  
   > **Note:** lz4 is broken for more recent versions of Python.

1. Clone [Epsilon](https://github.com/numworks/epsilon) version 20:
   ```sh
   git clone https://github.com/numworks/epsilon epsilon_simulator -b version-20
   ```

1. Remap the simulator keyboard inputs:
   ```sh
   python3 build/sim/remap_inputs.py epsilon_simulator
   ```

1. In the simulator directory, run `setup.sh`:
   ```sh
   cd epsilon_simulator
   ./build/setup.sh --only-simulator
   ```

### Building

To build the simulator, run:
```sh
cd epsilon_simulator

# linux/wsl 
make PLATFORM=simulator epsilon.bin -j [jobs]

# macos
make PLATFORM=simulator epsilon.app -j [jobs]
```
where `[jobs]` is the number of jobs to use when making.

This creates a binary/app file at `/epsilon_simulator/output/release/simulator/[your operating system]/epsilon`, with a file extension according to your operating system.

To run the app on the simulator, run:
```sh
just nwb-run
```
