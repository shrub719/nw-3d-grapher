# NumWorks 3D Grapher
> [Documentation](https://github.com/shrub719/3d-grapher-docs)

3D Grapher app for NumWorks Epsilon.  

For my A level Computer Science NEA.

## Installation

### NumWorks Calculator
1. Go to the [latest release](https://github.com/shrub719/fun-numworks-apps/releases/latest)
1. Download `nw_3d_grapher.nwa`
1. Connect to your calculator by USB
1. Go to the [Numworks Installer](https://my.numworks.com/apps) and click Connect (make sure your browser has WebUSB capability)
1. Upload `nw_3d_grapher.nwa` file and press Install

### NumWorks Calculator (with PBJ file)
The app supports importing 3D models as `.pbj`.

1. Go to the [latest release](https://github.com/shrub719/fun-numworks-apps/releases/latest)
1. Download `nw_3d_grapher_obj.nwa`  
   *Optionally, download the example `dog.pbj`*
1. Connect to your calculator by USB
1. Go to the [Numworks Installer](https://my.numworks.com/apps) and click Connect (make sure your browser has WebUSB capability)
1. Upload `nw_3d_grapher_obj.nwa` file
1. Upload any `.pbj` file to the External Data section
1. Press Install
<!-- TODO: what does this look like? -->

## Usage

<!-- TODO: controls, features -->

### Simulator
<!-- TODO: remapped sim controls -->

## Building

### Dependencies

<!-- TODO -->

### NumWorks Calculator

- To build the app, run:
  ```sh
  just build
  ```
  This creates a binary (`.nwa`) file at `/target/thumbv7em-none-eabihf/release/nw_3d_grapher`
- To load the app to the calculator, run:
  ```sh
  just load
  ```

### Simulator

<!-- TODO -->

### PBJ Files

The app supports importing 3D models as `.pbj`.
- To convert a `.obj` file to `.pbj`, run:
  ```sh
  just obj [file location] [object name]

  # Example usage:
  just obj obj/meshes/dog.obj dog
  ```
  This creates a `.pbj` file in `/target/obj/`.
- To load to the calculator with a converted PBJ, run:
  ```sh
  just load [object name]

  # Example usage:
  just load dog
  ```

## Licensing

As this repo (the NEA program) is part of my A level Computer Science coursework, it is under exam regulations. Therefore:
- **Until August 14 2026** all rights are reserved by me. No permission is granted to copy, use, modify, or distribute any part of this project during this period.
- **After August 15 2026**, the project is released under the [MIT License](LICENSE).

Dog model in `obj/Mesh_Beagle.obj` and `dog.pbj`: [Beagle](https://poly.pizza/m/0BnDT3T1wTE) by [Poly by Google](https://poly.pizza/u/Poly%20by%20Google) [[CC-BY](https://creativecommons.org/licenses/by/3.0/)] via Poly Pizza