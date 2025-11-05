use crate::{
    grapher::{
        mat::*,
        mesh::Domain,
    },
    trig::*,
    constants::limits::*
};
use heapless::Vec;

fn placeholder_func(x: f32, y: f32) -> f32 {
    sin(x * sin(y))
}

fn add_explicit_tris(tris: &mut Vec<Triangle3, { MAX_TRIS }>, x0: f32, y0: f32, dx: f32, dy: f32) {
    let mut vertices = [v!(0.0, 0.0, 0.0); 4];

    for i in 0..2 {
        for j in 0..2 {
            let x = x0 + i as f32 * dx;
            let y = y0 + j as f32 * dy;
            let z = placeholder_func(x, y);

            vertices[i*2 + j] = v!(x, y, z);
        }
    }

    let _ = tris.push(Triangle3([vertices[1], vertices[2], vertices[0]]));
    let _ = tris.push(Triangle3([vertices[1], vertices[2], vertices[3]]));
}

pub fn generate_mesh(tris: &mut Vec<Triangle3, { MAX_TRIS }>, domain: Domain) {
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

