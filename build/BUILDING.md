# NumWorks 3D Grapher - Building

## Building the app

### Setup build environment

<!-- TODO -->

### NumWorks calculator

To build the app, run:
```sh
just build
```
This creates a binary (`.nwa`) file at `/target/thumbv7em-none-eabihf/release/nw_3d_grapher`.

To load the app to the calculator, run:
```sh
just load
```

### PBJ files

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
This creates a binary (nwb) file at `/target/[your Rust host]/release/libnw_3d_grapher`, with a file extension according to your operating system.

<!-- TODO -->
To build the simulator, run:
```sh
just setup-sim
just build-sim
```
> **Note:** you may need to downgrade Python to version 3.10 in order to build.

This creates a binary/app file at `/epsilon_simulator/output/release/simulator/[your operating system]/epsilon`, with a file extension according to your operating system.

To run the app on the simulator, run:
```sh
just nwb-run
```

## Building the simulator

<!-- TODO -->

