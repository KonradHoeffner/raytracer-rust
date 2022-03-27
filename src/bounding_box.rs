use crate::triangle::Triangle;
use core::f32::{INFINITY, NEG_INFINITY};
use glam::f32::Vec3A;
use std::iter::Iterator;
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

    // Spannt eine Bounding Box um eine Menge von Dreiecken auf
    pub fn around(triangles: &mut dyn Iterator<Item = Triangle>) -> BoundingBox {
        let mut min = Vec3A::new(INFINITY, INFINITY, INFINITY);
        let mut max = Vec3A::new(NEG_INFINITY, NEG_INFINITY, NEG_INFINITY);
        for triangle in triangles {
            for point in triangle.p {
                min = min.min(point);
                max = max.max(point);
            }
        }
        return BoundingBox {
            min,
            max,
            triangles: Vec::new(),
            children: Vec::new(),
        };
    }
}
