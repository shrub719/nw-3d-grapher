use crate::{
    grapher::{
        mat::*
    },
    generator,
    constants::*,
    input::parser::Expr
};
use heapless::Vec;

fn get_projection_matrix(scale: f32) -> Matrix4 {
    Matrix4 ([
        [120.0, 0.0  , 0.0      , 160.0],
        [0.0  , 120.0, 0.0      , 110.0],
        [0.0  , 0.0  , 0.6/scale, 0.0  ],
        [0.0  , 0.0  , 0.0      , 1.0  ]
    ])
}

fn get_scale_matrix(scale: f32) -> Matrix4 {
    Matrix4 ([
        [scale, 0.0  , 0.0  , 0.0],
        [0.0  , scale, 0.0  , 0.0],
        [0.0  , 0.0  , scale, 0.0],
        [0.0  , 0.0  , 0.0  , 1.0]
    ])
}

#[derive(Clone, Copy)]
pub struct Domain {
    pub x0: f32,
    pub y0: f32,
    pub z0: f32,
    pub x1: f32,
    pub y1: f32,
    pub z1: f32,
    pub matrix: Matrix4
}
impl Domain {
    pub fn new() -> Self {
        Domain {
            x0: -10.0,
            y0: -10.0,
            z0: -10.0,
            x1: 10.0,
            y1: 10.0,
            z1: 10.0,
            matrix: Matrix4::new()
        }
    }

    pub fn translate(&mut self, trans_direction: Vector3) {
        if trans_direction.x.is_nan() {
            self.x0 = -10.0;
            self.y0 = -10.0;
            self.z0 = -10.0;
            self.x1 = 10.0;
            self.y1 = 10.0;
            self.z1 = 10.0;

            return
        }
        let dx = (self.x1 - self.x0) * 0.1 * trans_direction.x;
        let dy = (self.y1 - self.y0) * 0.1 * trans_direction.y;
        let dz = (self.z1 - self.z0) * 0.1 * trans_direction.z;

        self.x0 += dx;
        self.x1 += dx;
        self.y0 += dy;
        self.y1 += dy;
        self.z0 += dz;
        self.z1 += dz;
    }

    pub fn scale(&mut self, scale_direction: Vector3) {
        let dx = (self.x1 - self.x0) * (1.0 + scale_direction.x * settings::DOMAIN_SCALE_SPEED) / 2.0;
        let dy = (self.y1 - self.y0) * (1.0 + scale_direction.y * settings::DOMAIN_SCALE_SPEED) / 2.0;
        let dz = (self.z1 - self.z0) * (1.0 + scale_direction.z * settings::DOMAIN_SCALE_SPEED) / 2.0;

        let xm = (self.x0 + self.x1) / 2.0;
        let ym = (self.y0 + self.y1) / 2.0;
        let zm = (self.z0 + self.z1) / 2.0;

        self.x0 = xm - dx;
        self.x1 = xm + dx;
        self.y0 = ym - dy;
        self.y1 = ym + dy;
        self.z0 = zm - dz;
        self.z1 = zm + dz;
    }

    pub fn set_axes(&mut self, axes: &mut [Line3; 3]) {
        axes[0] = Line3([
            v!(self.x0, self.y0, self.z0),
            v!(self.x1, self.y0, self.z0)
        ]);
        axes[1] = Line3([
            v!(self.x0, self.y0, self.z0),
            v!(self.x0, self.y1, self.z0)
        ]);
        axes[2] = Line3([
            v!(self.x0, self.y0, self.z0),
            v!(self.x0, self.y0, self.z1)
        ]);
    }

    pub fn update_matrix(&mut self) {
        let dx = self.x1 - self.x0;
        let dy = self.y1 - self.y0;
        let dz = self.z1 - self.z0;

        let x_scale = 2.0 / dx;
        let y_scale = 2.0 / dy;
        let z_scale = 2.0 / dz;

        let x_trans = -self.x0 - dx/2.0;
        let y_trans = -self.y0 - dy/2.0;
        let z_trans = -self.z0 - dz/2.0;

        self.matrix = Matrix4 ( [
            [x_scale, 0.0    , 0.0    , x_trans*x_scale],
            [0.0    , y_scale, 0.0    , y_trans*y_scale],
            [0.0    , 0.0    , z_scale, z_trans*z_scale],
            [0.0    , 0.0    , 0.0    , 1.0            ]
        ] )
    }
}

pub struct Mesh {
    pub tris: Vec<Triangle3, { limits::MAX_TRIS }>,
    pub transformed_tris: Vec<RTriangle3, { limits::MAX_TRIS }>,
    // pub lines: Vec<Line>,
    pub axes: [Line3; 3],  // in order: x, y, z
    pub transformed_axes:  [RLine3; 3],
    pub domain: Domain,
    rotation: Quaternion,
    pub scale: f32
}
impl Mesh {
    pub fn new() -> Self {
        Self {
            tris: Vec::new(), 
            transformed_tris: Vec::new(),
            // lines:  Vec::with_capacity(limits::MAX_LINES), // TODO: transform lines
            axes: [Line3([v!(0.0, 0.0, 0.0); 2]); 3],
            transformed_axes: [RLine3([RVector3::new(0, 0, 0.0); 2]); 3],
            domain: Domain::new(),
            rotation: Quaternion::default(),
            scale: 0.5
        }
    }

    pub fn update_domain(&mut self, trans_direction: Vector3, scale_direction: Vector3) {
        self.domain.translate(trans_direction);
        self.domain.scale(scale_direction);
        self.domain.update_matrix();
        self.domain.set_axes(&mut self.axes);
    }

    pub fn update_rotation(&mut self, rotation_direction: Vector3, delta_time: f32) {
        if rotation_direction.x.is_nan() {
            self.rotation = Quaternion::default();
            return
        }

        let rotation_speed = settings::ROTATION_SPEED * delta_time;
        let x = rotation_direction.x * rotation_speed;
        let y = rotation_direction.y * rotation_speed;
        let z = rotation_direction.z * rotation_speed;
        
        self.rotation = Quaternion::from_angles(x, y, z) * self.rotation;
    }

    pub fn update_scale(&mut self, scale_change: f32, delta_time: f32) {
        self.scale += settings::SCALE_SPEED * scale_change * delta_time;
        if self.scale < 0.0 { self.scale = 0.0 }
    }

    fn get_matrix(&mut self) -> Matrix4 {
        let mut matrix = get_projection_matrix(self.scale);
        matrix *= self.rotation.get_rotation_matrix();
        matrix *= get_scale_matrix(self.scale);
        matrix *= self.domain.matrix;

        matrix
    }

    pub fn transform(&mut self) {
        let matrix = self.get_matrix();

        self.transformed_tris.clear();
        for tri in &self.tris {
            let _ = self.transformed_tris.push(*tri * matrix);
        }
        for i in 0..3 {
            self.transformed_axes[i] = self.axes[i] * matrix;
        }
    }

    pub fn generate_screen(&mut self, expr: &Expr) {
        let mut matrix = self.get_matrix(); 
        matrix = matrix.inverse();

        generator::raymarcher::generate_screen(
            expr,
            matrix
        );
    }
}
