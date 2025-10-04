# EXTREMELY sloppy way to convert obj file into rust code

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

with open("mesh.txt", "w") as f:
    for face in indices:
        print(face)
        v0 = vertices[face[0] - 1]
        v1 = vertices[face[1] - 1]
        v2 = vertices[face[2] - 1]
        f.write(f"""Triangle3([
    Vector3::new({v0[0]}, {v0[1]}, {v0[2]}),
    Vector3::new({v1[0]}, {v1[1]}, {v1[2]}),
    Vector3::new({v2[0]}, {v2[1]}, {v2[2]}),
]),\n""")

print("done")
