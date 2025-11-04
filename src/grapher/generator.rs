use crate::{
    grapher::{
        mat::*,
        mesh::Domain,
        tables::*
    },
    trig::*,
    constants::limits::*
};
use heapless::Vec;

fn test_curve(x: f32, y: f32) -> f32 {
    sin(x * sin(y))
}

fn test_surface(x: f32, y: f32, z: f32) -> f32 {
    x*x*x*x + 2.0*x*x*y*y + 2.0*x*x*z*z + y*y*y*y + 2.0*y*y*z*z + z*z*z*z + 8.0*x*y*z - 10.0*x*x - 10.0*y*y - 10.0*z*z + 20.0
}

fn implicit(v: Vector3) -> f32 {
    test_surface(v.x, v.y, v.z)
}

// TODO: clip
fn add_tri(tris: &mut Vec<Triangle3, { MAX_TRIS }>, vertices: [Vector3; 3]) {
    let _ = tris.push(Triangle3(vertices));
}

// TODO: split poly into tris
fn add_poly(tris: &mut Vec<Triangle3, { MAX_TRIS }>, poly: &[Vector3]) {
    for v in 1..poly.len()-1 {
        let _ = add_tri(tris, [poly[0], poly[v], poly[v+1]]);
    }
}

fn add_explicit_tris(tris: &mut Vec<Triangle3, { MAX_TRIS }>, x0: f32, y0: f32, dx: f32, dy: f32) {
    let mut vertices = [Vector3::new(0.0, 0.0, 0.0); 4];

    for i in 0..2 {
        for j in 0..2 {
            let x = x0 + i as f32 * dx;
            let y = y0 + j as f32 * dy;
            let z = test_curve(x, y);

            let index = if i == 0 { i*2 + j } else { i*2 + 1-j };
            vertices[index] = Vector3::new(x, y, z);
        }
    }

    add_poly(tris, &vertices);
}

pub fn explicit_func(tris: &mut Vec<Triangle3, { MAX_TRIS }>, domain: Domain) {
    let dx = (domain.x1 - domain.x0) / EXPLICIT_N as f32;
    let dy = (domain.y1 - domain.y0) / EXPLICIT_N as f32;

    for i in 0..EXPLICIT_N {
        for j in 0..EXPLICIT_N {
            let x = domain.x0 + dx * i as f32;
            let y = domain.y0 + dy * j as f32;
            
            add_explicit_tris(tris, x, y, dx, dy);
        }
    }
}

pub fn march_that_cube(
    tris: &mut Vec<Triangle3, { MAX_TRIS }>,
    v0: Vector3, v1: Vector3, v2: Vector3, v3: Vector3,
    v4: Vector3, v5: Vector3, v6: Vector3, v7: Vector3
) {
    let mut edge_index: usize = 0;
    
    if implicit(v0) < 0.0 { edge_index |= 1 << 0 };
    if implicit(v1) < 0.0 { edge_index |= 1 << 1 };
    if implicit(v2) < 0.0 { edge_index |= 1 << 2 };
    if implicit(v3) < 0.0 { edge_index |= 1 << 3 };
    if implicit(v4) < 0.0 { edge_index |= 1 << 4 };
    if implicit(v5) < 0.0 { edge_index |= 1 << 5 };
    if implicit(v6) < 0.0 { edge_index |= 1 << 6 };
    if implicit(v7) < 0.0 { edge_index |= 1 << 7 };

    let mut edge = EDGE_TABLE[edge_index];
    let mut vertices: [Vector3; 12];

    // TODO: just use a bunch of if statements
    let mut i = 0;
    while edge != 0 {
        let curr_edge = edge % 2;
        
        i += 1;
        edge = edge >> 1;
    }
}

pub fn implicit_func(tris: &mut Vec<Triangle3, { MAX_TRIS }>, domain: Domain) {
    let dx = (domain.x1 - domain.x0) / EXPLICIT_N as f32;
    let dy = (domain.y1 - domain.y0) / EXPLICIT_N as f32;
    let dz = (domain.z1 - domain.z0) / EXPLICIT_N as f32;

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
                    Vector3::new(x0, y0, z0),
                    Vector3::new(x1, y0, z0),
                    Vector3::new(x1, y1, z0),
                    Vector3::new(x0, y1, z0),
                    Vector3::new(x0, y0, z1),
                    Vector3::new(x1, y0, z1),
                    Vector3::new(x1, y1, z1),
                    Vector3::new(x0, y1, z1)
                );
            }
        }
    }
}
