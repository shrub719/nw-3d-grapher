use crate::{
    grapher::{
        mat::*,
        mesh::Domain,
    },
    generator::tables::*,
    constants::limits::*
};
use heapless::Vec;

fn placeholder_func(x: f32, y: f32, z: f32) -> f32 {
    x*x*x*x + 2.0*x*x*y*y + 2.0*x*x*z*z + y*y*y*y + 2.0*y*y*z*z + z*z*z*z + 8.0*x*y*z - 10.0*x*x - 10.0*y*y - 10.0*z*z + 20.0
    // 400.0 * (x*x*y*y + y*y*z*z + x*x*z*z) - (1.0-x*x-y*y-z*z)*(1.0-x*x-y*y-z*z)*(1.0-x*x-y*y-z*z)
}

fn test_surface(v: Vector3) -> f32 {
    placeholder_func(v.x, v.y, v.z)
}

fn interpolate_vertex(v0: Vector3, v1: Vector3, t0: f32, t1: f32) -> Vector3 {
    let mu = -t0 / (t1 - t0);
    
    v!(
        v0.x + mu * (v1.x - v0.x),
        v0.y + mu * (v1.y - v0.y),
        v0.z + mu * (v1.z - v0.z)
    )
}

fn march_that_cube(
    tris: &mut Vec<Triangle3, { MAX_TRIS }>,
    v0: Vector3, v1: Vector3, v2: Vector3, v3: Vector3,
    v4: Vector3, v5: Vector3, v6: Vector3, v7: Vector3
) {
    let t0 = test_surface(v0);
    let t1 = test_surface(v1);
    let t2 = test_surface(v2);
    let t3 = test_surface(v3);
    let t4 = test_surface(v4);
    let t5 = test_surface(v5);
    let t6 = test_surface(v6);
    let t7 = test_surface(v7);

    let mut cube_index: usize = 0;
    
    if t0 < 0.0 { cube_index |= 1 << 0 };
    if t1 < 0.0 { cube_index |= 1 << 1 };
    if t2 < 0.0 { cube_index |= 1 << 2 };
    if t3 < 0.0 { cube_index |= 1 << 3 };
    if t4 < 0.0 { cube_index |= 1 << 4 };
    if t5 < 0.0 { cube_index |= 1 << 5 };
    if t6 < 0.0 { cube_index |= 1 << 6 };
    if t7 < 0.0 { cube_index |= 1 << 7 };

    let edge = EDGE_TABLE[cube_index];
    let mut vertices: [Vector3; 12] = [v!(5.0, 0.0, 0.0); 12];

    if edge & (1 << 0) != 0 {
        vertices[0] = interpolate_vertex(v0, v1, t0, t1);
    } 
    if edge & (1 << 1) != 0 {
        vertices[1] = interpolate_vertex(v1, v2, t1, t2);
    } 
    if edge & (1 << 2) != 0 {
        vertices[2] = interpolate_vertex(v2, v3, t2, t3);
    } 
    if edge & (1 << 3) != 0 {
        vertices[3] = interpolate_vertex(v3, v0, t3, t0);
    } 
    if edge & (1 << 4) != 0 {
        vertices[4] = interpolate_vertex(v4, v5, t4, t5);
    } 
    if edge & (1 << 5) != 0 {
        vertices[5] = interpolate_vertex(v5, v6, t5, t6);
    } 
    if edge & (1 << 6) != 0 {
        vertices[6] = interpolate_vertex(v6, v7, t6, t7);
    } 
    if edge & (1 << 7) != 0 {
        vertices[7] = interpolate_vertex(v7, v4, t7, t4);
    } 
    if edge & (1 << 8) != 0 {
        vertices[8] = interpolate_vertex(v0, v4, t0, t4);
    } 
    if edge & (1 << 9) != 0 {
        vertices[9] = interpolate_vertex(v1, v5, t1, t5);
    } 
    if edge & (1 << 10) != 0 {
        vertices[10] = interpolate_vertex(v2, v6, t2, t6);
    } 
    if edge & (1 << 11) != 0 {
        vertices[11] = interpolate_vertex(v3, v7, t3, t7);
    }

    let triangle = TRI_TABLE[cube_index];

    let mut i = 0;
    while triangle[i] != 255 {
        let _ = tris.push(Triangle3([
            vertices[triangle[i] as usize],
            vertices[triangle[i+1] as usize],
            vertices[triangle[i+2] as usize]
        ]));
        i += 3;
    }
}

pub fn generate_mesh(tris: &mut Vec<Triangle3, { MAX_TRIS }>, domain: Domain) {
    let dx = (domain.x1 - domain.x0) / IMPLICIT_N as f32;
    let dy = (domain.y1 - domain.y0) / IMPLICIT_N as f32;
    let dz = (domain.z1 - domain.z0) / IMPLICIT_N as f32;

    for i in 0..IMPLICIT_N {
        for j in 0..IMPLICIT_N {
            for k in 0..IMPLICIT_N {
                let x0 = domain.x0 + dx * i as f32;
                let y0 = domain.y0 + dy * j as f32;
                let z0 = domain.z0 + dz * k as f32;
                let x1 = x0 + dx;
                let y1 = y0 + dy;
                let z1 = z0 + dz;
                
                march_that_cube(
                    tris,
                    v!(x0, y0, z0),
                    v!(x1, y0, z0),
                    v!(x1, y1, z0),
                    v!(x0, y1, z0),
                    v!(x0, y0, z1),
                    v!(x1, y0, z1),
                    v!(x1, y1, z1),
                    v!(x0, y1, z1)
                );
            }
        }
    }
}
