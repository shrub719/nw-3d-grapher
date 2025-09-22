# Notes

## Planning

### Structure
- Menu: Settings + math entry
- Parser: Turns math expression into a function
- Grapher: Turns a function into a mesh of points/tris
- Renderer: Rasterises tris
- Display (?): Controls calls to Grapher based on user input

### Transformation
- 4D matrix w/ translation from domain space to object space
- Matrix from object to camera space
    - Frustum?? How does foreshortening stuff work

## To do
- [x] File structure
- [x] Rasterise dummy camera space mesh
- [ ] Transform dummy object space mesh to camera space
- [x] Transform dummy domain space mesh to camera space
- [ ] Generate domain space mesh from explicit function
- [ ] Generate domain space mesh from implicit function with marching cubes
    - [ ] March those cubes (create cube loop that minimises redundant calculation)
    - [ ] Create tris by linearly interpolating vertex tests
