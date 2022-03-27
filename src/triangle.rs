use glam::f32::Vec3A;

#[test]
fn test() {
    let t = Triangle::new(
        Vec3A::new(-1.0, 0.0, 0.0),
        Vec3A::new(1.0, 0.0, 0.0),
        Vec3A::new(0.0, 1.0, 0.0),
    );
    println!("{:?}", t);
}

#[derive(Debug)]
struct Triangle {
    p: [Vec3A; 3],
    n: [Vec3A; 3],
}

impl Triangle {
    pub fn new(a: Vec3A, b: Vec3A, c: Vec3A) -> Self {
        let p: [Vec3A; 3] = [a, b, c];
        let normal = (p[1] - p[0]).cross(p[2] - p[1]);
        // we don't have information about neighbouring triangles, so set all normals to the same value
        let n: [Vec3A; 3] = [normal; 3];
        return Triangle { p, n };
    }
}
