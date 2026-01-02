use crate::{
    grapher::{
        Grapher,
        mat::*,
    },
    constants::limits::*
};

impl Grapher {
    fn add_explicit_tris(&mut self, x0: f32, y0: f32, dx: f32, dy: f32) {
        let mut vertices = [v!(0.0, 0.0, 0.0); 4];

        for i in 0..2 {
            for j in 0..2 {
                let x = x0 + i as f32 * dx;
                let y = y0 + j as f32 * dy;
                let z = (self.graph)(x, y, 0.0);

                vertices[i*2 + j] = v!(x, y, z);
            }
        }

        let _ = self.mesh.tris.push(Triangle3([vertices[1], vertices[2], vertices[0]]));
        let _ = self.mesh.tris.push(Triangle3([vertices[1], vertices[2], vertices[3]]));
    }

    pub fn generate_mesh_exp(&mut self) { 
        let dx = (self.mesh.domain.x1 - self.mesh.domain.x0) / EXPLICIT_N as f32;
        let dy = (self.mesh.domain.y1 - self.mesh.domain.y0) / EXPLICIT_N as f32;

        for i in 0..EXPLICIT_N {
            for j in 0..EXPLICIT_N {
                let x = self.mesh.domain.x0 + dx * i as f32;
                let y = self.mesh.domain.y0 + dy * j as f32;
                
                self.add_explicit_tris(x, y, dx, dy);
            }
        }
    }
}
