# To do
- [ ] Finish 3D engine
    - [ ] Refactor tris
        - [x] Use vertices instead of indices
        - [ ] Store + tranform normals
    - [ ] Use proper (configurable) projection matrix
    - [x] Use delta time in manipulation calculations
    - [x] Use quaternions in rotation
- [x] Generate domain space mesh from explicit function
    - [ ] Lines (and maybe inheritance from a 3D/R3D struct)
- [x] Generate domain space mesh from implicit function with marching cubes
    - [x] March those cubes (create cube loop that minimises redundant calculation)
    - [x] Create tris by linearly interpolating vertex tests
- [ ] UI
    - [ ] Settings menu (vsync, delta time)
- [ ] Pompeiu


## Better to do
- [x] Remove TODOs
- [x] Move generator and tables to a separate mod/folder
- [ ] Make shading not dependent on VIEW scale
- [x] Add credits to README, remove dog.obj

