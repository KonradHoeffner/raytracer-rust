use crate::color::Color;
use glam::f32::Vec3A;

struct LightSource {
    pos: Vec3A,
    color: Color,
}

impl LightSource {
    fn new() -> Self {
        Self {
            pos: Vec3A::new(0.0, 10.0, 0.0),
            color: Color::white(),
        }
    }
}
