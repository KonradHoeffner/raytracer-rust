use crate::camera::Camera;
use crate::color::Color;
use crate::light_source::LightSource;
use crate::material::Material;
use crate::scene::Scene;
use crate::triangle::Triangle;
use glam::f32::Vec3A;
use roxmltree::{Document, Node};
use std::collections::HashMap;
use std::fs;
use std::vec::Vec;

fn fatt(ele: &Node, att: &str) -> Option<f32> {
    ele.attribute(att)?.parse::<f32>().ok()
}

fn parseVector3(e: &Node) -> Option<Vec3A> {
    Some(Vec3A::new(fatt(e, "x")?, fatt(e, "y")?, fatt(e, "z")?))
}
#[test]
fn testGetVector3dFromElement() {
    let doc = Document::parse(r#"<punkt3 x="-0.7" y="1.4" z="0.38"/>"#).unwrap();
    let v: Vec3A = parseVector3(&doc.root_element()).unwrap();
    assert_eq!(v, Vec3A::new(-0.7, 1.4, 0.38));
}

fn parseColor(e: &Node) -> Option<Color> {
    Some(Color::new(
        fatt(e, "r")?,
        fatt(e, "g")?,
        fatt(e, "b")?,
        fatt(e, "a").unwrap_or(1.0),
    ))
}

fn itArray(it: &mut dyn Iterator<Item = Node>) -> Option<[Vec3A; 3]> {
    let mut elements = it.filter(|x| x.is_element());
    Some([
        parseVector3(&elements.next()?)?,
        parseVector3(&elements.next()?)?,
        parseVector3(&elements.next()?)?,
    ])
}

fn parseTriangle(e: &Node) -> Option<Triangle> {
    let mut it = e.children();
    let p = itArray(&mut it).unwrap();
    let n = itArray(&mut it).unwrap();
    Some(Triangle::with_normals(p, n))
}
#[test]
fn testParseTriangle() {
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
    assert_eq!(t.p[2], Vec3A::new(-0.7, 1.4, 0.38));
    assert_eq!(t.n[0], Vec3A::new(-0.4, 4.4, 0.4));
}

fn parseTriangulation(xml: &str) -> Option<(HashMap<String, Material>, Vec<Triangle>)> {
    let doc = Document::parse(xml).ok()?;
    let e = doc.root_element();
    let matEles = e.children().filter(|e| e.has_tag_name("material"));
    let mut materials: HashMap<String, Material> = HashMap::new();
    for m in matEles {
        let ambient = parseColor(&m.children().find(|e| e.has_tag_name("ambient"))?)?;
        let diffus = parseColor(&m.children().find(|e| e.has_tag_name("diffus"))?)?;
        let spiegelnd = parseColor(&m.children().find(|e| e.has_tag_name("spiegelnd"))?)?;
        let name = m.attribute("name")?;
        materials.insert(
            name.to_string(),
            Material {
                name: name.to_string(),
                glanz: m.attribute("glanzwert")?.parse::<f32>().ok()?,
                ambient,
                diffus,
                spiegelnd,
            },
        );
    }
    let triangleEles = e.children().filter(|e| e.has_tag_name("dreieck"));
    let mut triangles: Vec<Triangle> = Vec::new();
    for te in triangleEles {
        triangles.push(parseTriangle(&te)?);
    }
    Some((materials, triangles))
}
#[test]
fn testParseTriangulation() {
    const XML: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE triangulation SYSTEM "triangulation.dtd">
<triangulation>
<material glanzwert="1" name="blau"><ambient a="1.0" b="0.5" g="0.0" r="0.0"/>
<diffus a="0.5" b="0.5" g="0.0" r="0.0"/><spiegelnd a="1.0" b="0.5" g="0.0" r="0.0"/>
</material><material glanzwert="11" name="gelb"><ambient a="1.0" b="0.0" g="0.39" r="0.39"/>
<diffus a="1.0" b="0.5" g="0.5" r="0.78"/><spiegelnd a="1.0" b="0.0" g="1.0" r="1.0"/>
</material><dreieck material="gelb"><punkt1 x="-1.71" y="1.18" z="0.38"/><punkt2 x="-1.66" y="1.18" 
z="0.78"/>
<punkt3 x="-0.73" y="1.40" z="0.38"/>
<normale1 x="-0.42" y="4.40" z="0.43"/>
<normale2 x="-0.39" y="4.40" z="0.52"/>
<normale3 x="-0.42" y="4.40" z="0.43"/>
</dreieck><dreieck material="gelb"><punkt1 x="-1.66" y="1.18" z="0.78"/><punkt2 x="-0.71" y="1.40" z="0.55"/><punkt3 x="-0.73" y="1.40" z="0.38"/><normale1 x="-0.39" y="4.40" z="0.52"/><normale2 x="-0.72" y="4.29" z="0.56"/><normale3 x="-0.42" y="4.40" z="0.43"/></dreieck></triangulation>"#;
    let (materials, triangles) = parseTriangulation(XML).unwrap();
    assert_eq!(materials.get("blau").unwrap().ambient.a, 1.0);
    assert_eq!(triangles[0].p[0], Vec3A::new(-1.71, 1.18, 0.38));
}

fn parseCamera(e: &Node) -> Option<Camera> {
    let pos = parseVector3(&e.children().find(|e| e.has_tag_name("position"))?)?;
    let target = parseVector3(&e.children().find(|e| e.has_tag_name("target"))?)?;
    Some(Camera::new(pos, target))
}

fn parseLightsource(e: &Node) -> Option<LightSource> {
    let pos = parseVector3(&e.children().find(|e| e.has_tag_name("position"))?)?;
    let color = parseColor(&e.children().find(|e| e.has_tag_name("farbe"))?)?;
    Some(LightSource { pos, color })
}

fn parseScene(xml: &str) -> Option<Scene> {
    let doc = Document::parse(xml).ok()?;
    let e = doc.root_element();
    let triangulationSrc = e
        .children()
        .find(|e| e.has_tag_name("triangulation"))?
        .attribute("src")?;
    let txml = fs::read_to_string(triangulationSrc).ok()?;
    let (materials, triangles) = parseTriangulation(&txml).unwrap();
    let camera = parseCamera(&e.children().find(|e| e.has_tag_name("camera"))?)?;
    let beleuchtung = &e.children().find(|e| e.has_tag_name("beleuchtung"))?;
    let background = parseColor(&beleuchtung.children().find(|e| e.has_tag_name("hintergrundfarbe"))?)?;
    let ambient = parseColor(
        &beleuchtung.children()
            .find(|e| e.has_tag_name("ambientehelligkeit"))?,
    )?;
    Some(Scene {
        materials,
        triangles,
        background,
        ambient,
    })
}
#[test]
fn testParseScene() {
    const XML: &str = r#"
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE szene SYSTEM "szene.dtd">
<szene>
  <triangulation src="abgabetriangulation_high.xml"/>
  <fenster breite="320" hoehe="240"/>
  <raumteilung unterteilung="2"/>
  <kamera>
    <position x="-2.46" y="7.86" z="9.51"/>
    <ziel x="0.03" y="-1.27" z="-2.78"/>
    <fovy winkel="45.0"/>
  </kamera>
  <beleuchtung>
    <!-- Lichteigenschaften !-->
    <hintergrundfarbe b="0.5" g="0.2" r="0.2"/>
    <ambientehelligkeit b="1" g="1" r="1"/>
    <abschwaechung konstant="1" linear="0" quadratisch="1"/>
    <!-- Lichtquellen !-->
    <lichtquelle>
      <position x="8.07" y="6.48" z="-0.92"/>
      <farbe b="1.0" g="1.0" r="1.0"/>
    </lichtquelle>
    <lichtquelle>
      <position x="-3.00" y="8.35" z="10.65"/>
      <farbe b="1.0" g="1.0" r="1.0"/>
    </lichtquelle>
  </beleuchtung>
</szene>
"#;
    let scene = parseScene(XML).unwrap();
}
