use glam::f32::Vec3A;
use roxmltree::{Document, Node};

#[test]
fn testGetVector3dFromElement() {
    let doc = roxmltree::Document::parse(r#"<punkt3 x="-0.7" y="1.4" z="0.383050"/>"#).unwrap();
    let v: Vec3A = getVector3dFromElement(&doc.root_element()).unwrap();
    assert_eq!(v, Vec3A::new(-0.7, 1.4, 0.383050));
}

fn getVector3dFromElement(element: &Node) -> Option<Vec3A> {
    Some(Vec3A::new(
        element.attribute("x")?.parse::<f32>().ok()?,
        element.attribute("y")?.parse::<f32>().ok()?,
        element.attribute("z")?.parse::<f32>().ok()?,
    ))
}
