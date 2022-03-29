use crate::triangle::Triangle;
use core::f32::{INFINITY, NEG_INFINITY};
use glam::f32::Vec3A;
use std::iter::Iterator;
use std::vec::Vec;
//import SchnittEreignis;
const SPLIT_TRIANGLES_MIN: i8 = 4;

#[test]
fn testAround() {
    let b = BoundingBox::around(
        &mut vec![Triangle::new(
            Vec3A::new(1.0, 2.0, 3.0),
            Vec3A::new(-5.0, -7.0, 3.0),
            Vec3A::new(0.5, 4.0, 1.0),
        )]
        .into_iter(),
    );
    assert_eq!(b.min, Vec3A::new(-5.0, -7.0, 1.0));
    assert_eq!(b.max, Vec3A::new(1.0, 4.0, 3.0));
}

#[derive(Default)]
pub struct BoundingBox {
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
    pub fn new() -> Self {
        Self {
            min: Vec3A::new(0.0, 0.0, 0.0),
            max: Vec3A::new(0.0, 0.0, 0.0),
            triangles: Vec::new(),
            children: Vec::new(),
        }
    }

    // Spannt eine Bounding Box um eine Menge von Dreiecken auf
    pub fn around(triangles: &mut dyn Iterator<Item = Triangle>) -> Self {
        let mut min = Vec3A::new(INFINITY, INFINITY, INFINITY);
        let mut max = Vec3A::new(NEG_INFINITY, NEG_INFINITY, NEG_INFINITY);
        for triangle in triangles {
            for point in triangle.p {
                min = min.min(point);
                max = max.max(point);
            }
        }
        Self {
            min,
            max,
            triangles: Vec::new(),
            children: Vec::new(),
        }
    }
}
