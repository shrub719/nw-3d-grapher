# EXTREMELY sloppy way to convert obj file into rust code

import struct

vertices = []
indices = []

with open("mesh.obj", "r") as o:
    lines = o.readlines()
    for line in lines:
        if line[0:2] == "v ":
            print(line)
            line = line[2:]
            v = line.split()
            vertices.append((
                float(v[0]),
                float(v[1]),
                float(v[2])
            ))
        elif line[0] == "f":
            line = line[2:]
            face = line.split()
            f = [vertex.split("/")[0] for vertex in face]
            if len(f) == 3:
                indices.append((
                    int(f[0]),
                    int(f[1]),
                    int(f[2])
                ))
            elif len(f) == 4:
                indices.append((
                    int(f[0]),
                    int(f[1]),
                    int(f[2])
                ))
                indices.append((
                    int(f[0]),
                    int(f[2]),
                    int(f[3])
                ))

print("read tris")

tris = []
for face in indices:
    print(face)
    v0 = vertices[face[0] - 1]
    v1 = vertices[face[1] - 1]
    v2 = vertices[face[2] - 1]
    tris.append([v0, v1, v2])

print("assembled tris")

with open("mesh.pbj", "wb") as f:
    for tri in tris:
        # Flatten the triangle’s coordinates
        flat = [coord for vertex in tri for coord in vertex]
        # Pack as 9 floats, little endian
        f.write(struct.pack("<9f", *flat))

print("done")
