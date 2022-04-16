use crate::color::Color;
use crate::material::Material;
use crate::triangle::Triangle;
use crate::camera::Camera;
use std::collections::HashMap;

pub struct Scene {
    /*
    fnTriangulation: String;
    fnSzene: String;
    bildBreite: i16;
    bildHoehe: i16;

    //double r,g,b;
    */
    pub background: Color,
    pub ambient: Color,
    /*
    double abschwaechung_konstant;
    double abschwaechung_linear;
    double abschwaechung_quadratisch;

    int unterteilung; // Unterteilung für das Raumteilungsverfahren
    */
    pub camera: Camera,
    //boundingBox: BoundingBox,
    pub materials: HashMap<String, Material>,
    pub triangles: Vec<Triangle>,
    /*list<Dreieck*> dreiecke,
    list<Lichtquelle*> lichtquellen,
    */
}

impl Scene {
    /*
    pub fn load(filename: &str) -> Self {
        Self {}
    }

    pub fn default() -> Self {
        return Self::load("abgabeszene.xml");
    }
    */
    /*pub fn test() -> Self {
        Self {
            camera: Camera::new(),
            boundingBox: BoundingBox::new(),
        }
    }*/
}
