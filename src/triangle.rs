use glam::f32::Vec3A;

#[test]
fn testNormal() {
    let t = Triangle::new(
        Vec3A::new(-1.0, 0.0, 0.0),
        Vec3A::new(1.0, 0.0, 0.0),
        Vec3A::new(0.0, 1.0, 0.0),
    );
    println!("{:?}", t);
    let normal = t.n[0];
    assert!(normal.x == 0.0);
    assert!(normal.y == 0.0);
    assert!(normal.z != 0.0);
}

#[derive(Debug)]
pub struct Triangle {
    pub p: [Vec3A; 3],
    pub n: [Vec3A; 3],
}

impl Triangle {
    pub fn from_array(p: [Vec3A; 3]) -> Self {
        let normal = (p[1] - p[0]).cross(p[2] - p[1]);
        // we don't have information about neighbouring triangles, so set all normals to the same value
        let n: [Vec3A; 3] = [normal; 3];
        Triangle { p, n }
    }
    pub fn with_normals(p: [Vec3A; 3], n: [Vec3A; 3]) -> Self {
        Self { p, n }
    }
    pub fn new(a: Vec3A, b: Vec3A, c: Vec3A) -> Self {
        Self::from_array([a, b, c])
    }
}
