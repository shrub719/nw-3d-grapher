import struct, sys

def info(string):
    print("    " + string)


def v_to_vertex(line):
    vertex = line[2:].split()
    return tuple(float(coord) for coord in vertex)


# splits n-gons into triangles
def poly_to_tris(indices):
    tris = []

    for i in range(1, len(indices)-1):
        tris.append(tuple(int(indices[j]) for j in [0, i, i+1]))
    
    return tris


def f_to_face(line):
    face = line[2:].split()
    indices = [index.split("/")[0] for index in face]

    if len(indices) == 3:
        return [tuple(int(index) for index in indices)]
    
    return poly_to_tris(indices)


def extract_obj(filename):
    vertices, faces = [], []

    with open(filename, "r") as o:
        lines = o.readlines()
        for line in lines:
            if line[0:2] == "v ":
                vertices.append(v_to_vertex(line))
            elif line[0] == "f":
                faces.extend(f_to_face(line))

    return vertices, faces


def normalise(vertices):
    n = len(vertices)

    # finds middle coordinate
    max_coord = lambda i: max(vertex[i] for vertex in vertices)
    min_coord = lambda i: min(vertex[i] for vertex in vertices)
    middle = lambda i: (max_coord(i) + min_coord(i)) / 2
    x = middle(0)
    y = middle(1)
    z = middle(2)

    # centres object
    for i, vertex in enumerate(vertices):
        vertices[i] = (
            vertex[0] - x,
            vertex[1] - y,
            vertex[2] - z
        )

    # finds average length
    length = 0
    for vertex in vertices:
        length += vertex[0]**2 + vertex[1]**2 + vertex[2]**2
    length = (length/n)**0.5
    if length == 0: length = 1

    # attempts to normalise object
    for i, vertex in enumerate(vertices):
        vertices[i] = (
            vertex[0] / length * 10,
            -vertex[1] / length * 10,
            -vertex[2] / length * 10    # flip orientation
        )

    return vertices


# turns faces with indices into faces with vertices
def obj_to_tris(vertices, faces):
    tris = []
    for face in faces:
        tri = [vertices[index - 1] for index in face]
        tris.append(tri)
    
    return tris


def pack_tris(tris, filename):
    with open(filename, "wb") as f:
        for tri in tris:
            flat = [coord for vertex in tri for coord in vertex]
            f.write(struct.pack("<9f", *flat))


inpt = sys.argv[1]
name = sys.argv[2]
output = "target/obj/" + name + ".pbj"

vertices, faces = extract_obj(inpt)

vertices = normalise(vertices)

tris = obj_to_tris(vertices, faces)

pack_tris(tris, output)

info(f"created pbj '{name}' ({len(tris)} tris)")
