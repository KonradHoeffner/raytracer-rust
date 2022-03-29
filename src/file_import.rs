use crate::color::Color;
use crate::triangle::Triangle;
use glam::f32::Vec3A;
use roxmltree::{Document, Node};

fn fatt(ele: &Node, att: &str) -> Option<f32> {
    ele.attribute(att)?.parse::<f32>().ok()
}

fn getVector3dFromElement(e: &Node) -> Option<Vec3A> {
    Some(Vec3A::new(fatt(e, "x")?, fatt(e, "y")?, fatt(e, "z")?))
}
#[test]
fn testGetVector3dFromElement() {
    let doc = Document::parse(r#"<punkt3 x="-0.7" y="1.4" z="0.383050"/>"#).unwrap();
    let v: Vec3A = getVector3dFromElement(&doc.root_element()).unwrap();
    assert_eq!(v, Vec3A::new(-0.7, 1.4, 0.383050));
}

fn getFarbeFromElement4d(e: &Node) -> Option<Color> {
    Some(Color::new(
        fatt(e, "r")?,
        fatt(e, "g")?,
        fatt(e, "b")?,
        fatt(e, "a")?,
    ))
}

fn itArray(it: &mut dyn Iterator<Item = Node>) -> Option<[Vec3A; 3]> {
    it.next();
    //println!("blabla {:?}",it.next().unwrap());
    Some([
        getVector3dFromElement(&it.next()?)?,
        getVector3dFromElement(&it.next()?)?,
        getVector3dFromElement(&it.next()?)?,
    ])
}

fn parseTriangle(e: &Node) -> Option<Triangle> {
    let mut it = e.children();
    let p = itArray(&mut it).unwrap();
    let n = itArray(&mut it).unwrap();
    Some(Triangle::with_normals(p, n))
}
#[test]
fn testTriangle() {
    const XML: &str = r#"<dreieck material="gelb">
<punkt1 x="-1.7" y="1.18" z="0.38"/>
<punkt2 x="-1.6" y="1.18" z="0.78"/>
<punkt3 x="-0.7" y="1.40" z="0.38"/>
<normale1 x="-0.4" y="4.4" z="0.4"/>
<normale2 x="-0.3" y="4.4" z="0.5"/>
<normale3 x="-0.4" y="4.4" z="0.4"/>
</dreieck>"#;
    let doc = Document::parse(XML).unwrap();
    let t: Triangle = parseTriangle(&doc.root_element()).unwrap();
}
