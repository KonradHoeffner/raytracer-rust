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
use std::error::Error;
use std::fmt;

fn fatt(ele: &Node, att: &str) -> Result<f32,anyhow::Error> {
    ele.attribute(att).ok_or(anyhow!("Missing float attribute: {}",att))?.parse::<f32>()?;
}

fn satt(ele: &Node, att: &str) -> Result<String,anyhow::Error> {
    ele.attribute(att).ok_or(anyhow!("Missing string attribute: {}",att));
}

fn parseVector3(e: &Node) -> Result<Vec3A,anyhow::Error> {
    Ok(Vec3A::new(fatt(e, "x")?, fatt(e, "y")?, fatt(e, "z")?))
}
#[test]
fn testGetVector3dFromElement() {
    let doc = Document::parse(r#"<punkt3 x="-0.7" y="1.4" z="0.38"/>"#).unwrap();
    let v: Vec3A = parseVector3(&doc.root_element()).unwrap();
    assert_eq!(v, Vec3A::new(-0.7, 1.4, 0.38));
}

fn parseColor(e: &Node) -> Result<Color,anyhow::Error> {
    Ok(Color::new(
        fatt(e, "r")?,
        fatt(e, "g")?,
        fatt(e, "b")?,
        fatt(e, "a").unwrap_or(1.0),
    ))
}
/*
fn itArray(it: &mut dyn Iterator<Item = Node>) -> Result<[Vec3A; 3],anyhow::Error> {
    let mut elements = it.filter(|x| x.is_element());
    Ok([
        parseVector3(&elements.next()?)?,
        parseVector3(&elements.next()?)?,
        parseVector3(&elements.next()?)?,
    ])
}
*/
fn parseTriangle(e: &Node) -> Result<Triangle,anyhow::Error> {
    let mut it = e.children();
    let p = itArray(&mut it).unwrap();
    let n = itArray(&mut it).unwrap();
    Ok(Triangle::with_normals(p, n))
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

fn parseTriangulation(xml: &str) -> Result<(HashMap<String, Material>, Vec<Triangle>),anyhow::Error> {
    let doc = Document::parse(xml)?;
    let e = doc.root_element();
    let matEles = e.children().filter(|e| e.has_tag_name("material"));
    let mut materials: HashMap<String, Material> = HashMap::new();
    for m in matEles {
        let ambient = parseColor(childNode(&m,"ambient")?)?;
        let diffus = parseColor(childNode(&m,"diffus")?)?;
        let spiegelnd = parseColor(childNode(&m,"spiegelnd")?)?;
        let name = satt(&m,"name")?;
        materials.insert(
            name.to_string(),
            Material {
                name: name.to_string(),
                glanz: fatt(&m,"glanzwert")?,
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
    Ok((materials, triangles))
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

fn childNode<'a>(e: &'a Node, tagName: &'static str) -> Result<&'a Node<'a, 'a>, anyhow::Error>
{
    &e.children().find(|e| e.has_tag_name("position")).ok_or(anyhow!("Element has no child element {}",tagName))
}

fn parseCamera(e: &Node) -> Result<Camera,anyhow::Error> {
    let pos = parseVector3(childNode(&e,"position")?)?;
    let target = parseVector3(childNode(&e,"target")?)?;
    Ok(Camera::new(pos, target))
}

fn parseLightsource(e: &Node) -> Result<LightSource,anyhow::Error> {
    let pos = parseVector3(childNode(&e,"position")?)?;
    let color = parseColor(childNode(&e,"farbe")?)?;
    Ok(LightSource { pos, color })
}

fn parseScene(xml: &str) -> Result<Scene,anyhow::Error> {
    let doc = Document::parse(xml)?;
    let e = doc.root_element();
    let triangulationSrc = satt(childNode(&e,"triangulation")?,"src")?;
    let txml = fs::read_to_string(triangulationSrc)?;
    let (materials, triangles) = parseTriangulation(&txml).unwrap();
    let camera = parseCamera(childNode(&e,"camera")?)?;
    let beleuchtung = childNode(&e,"beleuchtung")?;
    let background = parseColor(childNode(&beleuchtung,"hintergrundfarbe")?)?;
    let ambient = parseColor(
        childNode(&beleuchtung,"ambientehelligkeit")?,
    )?;
    Ok(Scene {
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
