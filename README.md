# NumWorks 3D Grapher
> [Documentation](https://github.com/shrub719/nw-3d-grapher-docs)

3D Grapher app for NumWorks Epsilon.  

For my A level Computer Science NEA.


## Installation

### NumWorks calculator

1. Go to the [latest release](https://github.com/shrub719/nw-3d-grapher/releases/latest)

1. Download `nw_3d_grapher.nwa`

1. Connect to your calculator by USB

1. Go to the [NumWorks Installer](https://my.numworks.com/apps) and click **Connect** (make sure your browser has WebUSB capability)

1. Upload `nw_3d_grapher.nwa`

1. Press **Install**

### Simulator

1. Go to the [latest release](https://github.com/shrub719/fun-numworks-apps/releases/latest)

1. Download the `.nwb` file for your operating system

    > **Note:** on Windows, use WSL.

1. Get the simulator for your operating system
   > **Note:** NumWorks does not allow simulators to be redistributed, so you will have to [patch and build the simulator yourself](build/BUILDING.md#building-the-simulator).

1. Run the `.nwb` file with the simulator in your terminal:

   ```sh
   # linux/wsl
   ./epsilon_linux.bin --nwb ./nw_3d_grapher_linux.nwb

   # macos
   ./epsilon_macos.app/Contents/MacOS/Epsilon --nwb ./nw_3d_grapher_macos.nwb
   ```

### PBJ Files

To import 3D models to your NumWorks calculator or simulator, use [3Dino](https://github.com/shrub719/nw-3dino) instead.


## Usage

### NumWorks calculator

<!-- TODO: controls, features -->

### Simulator

<!-- TODO: remapped sim controls -->


## Building

See [BUILDING.md](build/BUILDING.md) for instructions on how to build the app or the simulator.


## Licensing and credits

As this repo (the NEA program) is part of my A level Computer Science coursework, it is under exam regulations. Therefore:
- **Until August 14 2026** all rights are reserved by me. No permission is granted to copy, use, modify, or distribute any part of this project during this period.
- **After August 15 2026**, the project is released under the [MIT License](LICENSE).

This project is a third-party app and is not affiliated with NumWorks. NumWorks is a registered trademark of NumWorks SAS.

Thanks to:
   - [yannis300307](https://github.com/yannis30030) for parts of the extended EADK, and for [NumcraftRust](https://github.com/yannis300307/NumcraftRust) which inspired this project
   - [Paul Borke](https://paulbourke.net) for the marching cubes lookup tables [here](https://paulbourke.net/geometry/polygonise/)
