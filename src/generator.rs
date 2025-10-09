use crate::mat::*;
use crate::trig::*;
use crate::mesh::Domain;
use crate::config::limits::*;
#[cfg(target_os = "none")]
use alloc::vec::Vec;

fn test_curve(x: f32, y: f32) -> f32 {
    sin(x * sin(y))
}

pub fn explicit_func(tris: &mut Vec<Triangle3>, domain: Domain) {
    let dx = (domain.x1 - domain.x0) / SAMPLE_N as f32;
    let dy = (domain.y1 - domain.y0) / SAMPLE_N as f32;

    let mut grid = [Vector3::new(0.0, 0.0, 0.0); SAMPLE_N * SAMPLE_N];
    for i in 0..SAMPLE_N {
        for j in 0..SAMPLE_N {
            let index: usize = SAMPLE_N * i + j;
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

    for i in 0..SAMPLE_N-1 {
        for j in 0..SAMPLE_N-1 {
            let v0: usize = SAMPLE_N * i + j;
            let v1: usize = SAMPLE_N * (i + 1) + j;
            let v2: usize = SAMPLE_N * (i + 1) + j + 1;
            let v3: usize = SAMPLE_N * i + j + 1;

            tris.push(Triangle3([
                grid[v0], grid[v1], grid[v2]
            ]));
            tris.push(Triangle3([
                grid[v0], grid[v3], grid[v2]
            ]));
        }
    }
}