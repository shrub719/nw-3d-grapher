use crate::eadk::timing::millis;

pub struct Timer {
    pub delta_time: f32,
    prev_time: u64
}
impl Timer {
    pub fn new() -> Self {
        Timer {
            delta_time: 0.1,
            prev_time: millis()
        }
    }

    pub fn update(&mut self) {
        let now = millis();
        self.delta_time = (now - self.prev_time) as f32 / 1000.0;
        self.prev_time = now;
    }

    pub fn get_fps(&self) -> f32 {
        1.0 / self.delta_time
    }
}