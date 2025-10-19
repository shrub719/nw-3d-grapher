use crate::mat::*;
use crate::trig::*;
use crate::mesh::Domain;
use crate::config::limits::*;
#[cfg(target_os = "none")]
use alloc::vec::Vec;

fn test_curve(x: f32, y: f32) -> f32 {
    sin(x * sin(y))
}

// TODO: clip
fn add_tri(tris: &mut Vec<Triangle3>, mut vertices: [Vector3; 3]) {
    tris.push(Triangle3(vertices));
}

// TODO: split poly into tris
fn add_poly(tris: &mut Vec<Triangle3>) {

}

fn add_explicit_tris(tris: &mut Vec<Triangle3>, x0: f32, y0: f32, dx: f32, dy: f32) {
    let mut vertices = [Vector3::new(0.0, 0.0, 0.0); 4];

    for i in 0..2 {
        for j in 0..2 {
            let x = x0 + i as f32 * dx;
            let y = y0 + j as f32 * dy;
            let z = test_curve(x, y);

            vertices[i*2 + j] = Vector3::new(x, y, z);
        }
    }

    add_tri(tris, [vertices[1], vertices[2], vertices[0]]);
    add_tri(tris, [vertices[1], vertices[2], vertices[3]]);
}

pub fn explicit_func(tris: &mut Vec<Triangle3>, domain: Domain) {
    let dx = (domain.x1 - domain.x0) / SAMPLE_N as f32;
    let dy = (domain.y1 - domain.y0) / SAMPLE_N as f32;

    for i in 0..SAMPLE_N {
        for j in 0..SAMPLE_N {
            let x = domain.x0 + dx * i as f32;
            let y = domain.y0 + dy * j as f32;
            
            add_explicit_tris(tris, x, y, dx, dy);
        }
    }
}