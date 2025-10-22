# NumWorks 3D Grapher - Building

## Building the app

### Setup build environment <!-- TODO -->

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

### NumWorks calculator with PBJ files

The app supports importing 3D models as `.pbj`.

To convert a `.obj` file to `.pbj`, run:
```sh
just obj [file location] [object name]

# Example usage:
just obj obj/meshes/dog.obj dog
```
This creates a `.pbj` file in `/target/obj/`.

To build the app with PBJ support, run:
```sh
just build o
```

To load the app to the calculator with a converted PBJ, run:
```sh
just load [object name]

# Example usage:
just load dog
```

### Simulator

To build the app for the simulator, run:
```sh
just nwb-build
```
This creates a binary (`.nwb`) file at `/target/[your Rust host]/release/libnw_3d_grapher_sim`, with a file extension according to your operating system.


## Building the simulator

### Setup build environment

1. Install the [Epsilon SDK](https://www.numworks.com/engineering/software/build/)
1. Install [Python 3.10](https://www.python.org/downloads/release/python-3100/)  
   *Makefile is broken for recent versions of Python.* <!-- TODO: what is broken? -->
1. Run:

   ```sh
   just setup-sim
   ```

   This will:
   1. Clone Epsilon
   1. Run `build/setup.sh`
   1. Remap the simulator controls

### Building

To build the simulator, run:
```sh
just build-sim [jobs]
```
where `[jobs]` is the number of jobs to use when making.

This creates a binary/app file at `/epsilon_simulator/output/release/simulator/[your operating system]/epsilon`, with a file extension according to your operating system.

To run the app on the simulator, run:
```sh
just nwb-run
```
