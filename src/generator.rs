use crate::mat::*;
use crate::trig::*;
use crate::mesh::Domain;
#[cfg(target_os = "none")]
use alloc::vec::Vec;

fn test_curve(x: f32, y: f32) -> f32 {
    (x*x*x * y - y*y*y * x) / 390.0 
}

pub fn explicit_func(tris: &mut Vec<Triangle3>, domain: Domain) {
    let dx = (domain.x1 - domain.x0) / 10.0;
    let dy = (domain.y1 - domain.y0) / 10.0;

    let mut grid = [Vector3::new(0.0, 0.0, 0.0); 100];
    for i in 0..10 {
        for j in 0..10 {
            let index: usize = 10 * i + j;
            let x = domain.x0 + dx * i as f32;
            let y = domain.y0 + dy * j as f32;
            let mut z = test_curve(x, y);
            // TODO: have tris clip against the domain
            if z > domain.z1 {
                z = domain.z1;
            } else if z < domain.z0 {
                z = domain.z0;
            }

            grid[index] = Vector3::new(x, y, z);
        }
    }

    for i in 0..9 {
        for j in 0..9 {
            let v0: usize = 10 * i + j;
            let v1: usize = 10 * (i + 1) + j;
            let v2: usize = 10 * (i + 1) + j + 1;
            let v3: usize = 10 * i + j + 1;

            tris.push(Triangle3([
                grid[v0], grid[v1], grid[v2]
            ]));
            tris.push(Triangle3([
                grid[v0], grid[v3], grid[v2]
            ]));
        }
    }
}