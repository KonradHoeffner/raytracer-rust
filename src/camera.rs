/* Kameraeinstellungen (Position, Orientierung, Field of View) */

use glam::f32::Vec3A;

struct Camera {
    // Alles was näher an der Kamera ist wird geclippt
    clippingNear: f32,
    // Alles was weiter weg von der Kamera ist wird geclippt
    clippingFar: f32,
    // Winkel in Bogenmaß, der den Sichtwinkel in Y - Richtung angibt. Muss zwischen 0 und pi liegen.
    //fovy: f32,
    // Winkel in Bogenmaß, der den Sichtwinkel in X - Richtung angibt. Muss zwischen 0 und pi liegen.
    //fovx: f32,
    // position of the camera
    pos: Vec3A,
    // Where the camera points to
    target: Vec3A,
    // points upward
    up: Vec3A,
}

impl Camera {
    fn new() -> Self {
        Self {
            clippingNear: 0.01,
            clippingFar: 1000.0,
            pos: Vec3A::new(0.0, 0.0, 5.0),
            target: Vec3A::new(0.0, 0.0, 0.0),
            up: Vec3A::new(0.0, 1.0, 0.0),
        }
    }

    // liefert normierten Vektor zurück der von der Kameraposition zum Kameratarget zeigt
    pub fn getSichtVektor(&self) -> Vec3A {
        let v: Vec3A = self.target - self.pos;
        return v.normalize();
    }

    // liefert normierten Vektor zurück der nach rechts zeigt
    pub fn getRechts(&self) -> Vec3A {
        let v: Vec3A = self.up.cross(self.getSichtVektor());
        return v.normalize();
    }
}
