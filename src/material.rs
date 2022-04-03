use crate::color::Color;

pub struct Material {
    pub name: String,
    pub ambient: Color,
    pub diffus: Color,
    pub spiegelnd: Color,
    pub glanz: f32,
}

impl Material {
    pub fn isSpiegelnd(&self) -> bool {
        self.spiegelnd.a != 1.0
    }
    pub fn isTransparent(&self) -> bool {
        self.diffus.a != 1.0
    }
}
