use crate::color::Color;
use glam::f32::Vec3A;

pub struct LightSource {
    pub pos: Vec3A,
    pub color: Color,
}

impl LightSource {
    fn new() -> Self {
        Self {
            pos: Vec3A::new(0.0, 10.0, 0.0),
            color: Color::white(),
        }
    }
}
