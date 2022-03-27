use crate::triangle::Triangle;
use glam::f32::Vec3A;
use std::vec::Vec;

//import SchnittEreignis;
const SPLIT_TRIANGLES_MIN: i8 = 4;

#[derive(Default)]
struct BoundingBox {
    //distance: f32,
    //unterteilung: i32,
    // min enthält die minimalen - , max die maximalen x,y und z - Werte für die eine Mitgliedschaft in der Box gilt
    min: Vec3A,
    max: Vec3A,
    // Alle Dreiecke, die (teilweise) in der Box enthalten sind
    triangles: Vec<Triangle>,
    children: Vec<BoundingBox>,
}

impl BoundingBox {
    pub fn new() -> BoundingBox {
        return BoundingBox {
            min: Vec3A::new(0.0, 0.0, 0.0),
            max: Vec3A::new(0.0, 0.0, 0.0),
            triangles: Vec::new(),
            children: Vec::new(),
        };
    }
}
