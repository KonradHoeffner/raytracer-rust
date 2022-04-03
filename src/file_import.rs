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


use anyhow::anyhow;

macro_rules! childNode{
 ($e:expr,$t:expr)=>{
    $e.children().find(|e| e.has_tag_name($t)).ok_or(anyhow!("Element has no child element {}",$t))
 }
}

fn fatt(ele: &Node, att: &str) -> Result<f32,anyhow::Error> {
    Ok(ele.attribute(att).ok_or(anyhow!("Missing float attribute: {}",att))?.parse::<f32>()?)
}

fn satt(ele: &Node, att: &str) -> Result<String,anyhow::Error> {
    Ok(ele.attribute(att).ok_or(anyhow!("Missing string attribute: {}",att))?.to_string())
}

fn parse_vector3(e: &Node) -> Result<Vec3A,anyhow::Error> {
    Ok(Vec3A::new(fatt(e, "x")?, fatt(e, "y")?, fatt(e, "z")?))
}
#[test]
fn test_get_vector3d_from_element() {
    let doc = Document::parse(r#"<punkt3 x="-0.7" y="1.4" z="0.38"/>"#).unwrap();
    let v: Vec3A = parse_vector3(&doc.root_element()).unwrap();
    assert_eq!(v, Vec3A::new(-0.7, 1.4, 0.38));
}

fn parse_color(e: &Node) -> Result<Color,anyhow::Error> {
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
        parse_vector3(&elements.next()?)?,
        parse_vector3(&elements.next()?)?,
        parse_vector3(&elements.next()?)?,
    ])
}
*/
fn parse_triangle(e: &Node) -> Result<Triangle,anyhow::Error> {
    let p = [parse_vector3(&childNode!(&e,"punkt1")?)?,parse_vector3(&childNode!(&e,"punkt2")?)?,parse_vector3(&childNode!(&e,"punkt3")?)?];
    let n = [parse_vector3(&childNode!(&e,"normale1")?)?,parse_vector3(&childNode!(&e,"normale2")?)?,parse_vector3(&childNode!(&e,"normale3")?)?];
    Ok(Triangle::with_normals(p, n))
}
#[test]
fn test_parse_triangle() {
    const XML: &str = r#"<dreieck material="gelb">
<punkt1 x="-1.7" y="1.18" z="0.38"/>
<punkt2 x="-1.6" y="1.18" z="0.78"/>
<punkt3 x="-0.7" y="1.40" z="0.38"/>
<normale1 x="-0.4" y="4.4" z="0.4"/>
<normale2 x="-0.3" y="4.4" z="0.5"/>
<normale3 x="-0.4" y="4.4" z="0.4"/>
</dreieck>"#;
    let doc = Document::parse(XML).unwrap();
    let t: Triangle = parse_triangle(&doc.root_element()).unwrap();
    assert_eq!(t.p[2], Vec3A::new(-0.7, 1.4, 0.38));
    assert_eq!(t.n[0], Vec3A::new(-0.4, 4.4, 0.4));
}

fn parse_triangulation(xml: &str) -> Result<(HashMap<String, Material>, Vec<Triangle>),anyhow::Error> {
    let doc = Document::parse(xml)?;
    let e = doc.root_element();
    let matEles = e.children().filter(|e| e.has_tag_name("material"));
    let mut materials: HashMap<String, Material> = HashMap::new();
    for m in matEles {
        let ambient = parse_color(&childNode!(&m,"ambient")?)?;
        let diffus = parse_color(&childNode!(&m,"diffus")?)?;
        let spiegelnd = parse_color(&childNode!(&m,"spiegelnd")?)?;
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
        triangles.push(parse_triangle(&te)?);
    }
    Ok((materials, triangles))
}
#[test]
fn test_parse_triangulation() {
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
    let (materials, triangles) = parse_triangulation(XML).unwrap();
    assert_eq!(materials.get("blau").unwrap().ambient.a, 1.0);
    assert_eq!(triangles[0].p[0], Vec3A::new(-1.71, 1.18, 0.38));
}



/*
fn childNode<'a>(e: &'a Node, tagName: &'static str) -> Result<&'a Node<'a, 'a>, anyhow::Error>
{
    Ok(&e.children().find(|e| e.has_tag_name("position")).ok_or(anyhow!("Element has no child element {}",tagName))?)
}
*/
fn parse_camera(e: &Node) -> Result<Camera,anyhow::Error> {
    let pos = parse_vector3(&childNode!(&e,"position")?)?;
    let target = parse_vector3(&childNode!(&e,"ziel")?)?;
    Ok(Camera::new(pos, target))
}

fn parse_lightsource(e: &Node) -> Result<LightSource,anyhow::Error> {
    let pos = parse_vector3(&childNode!(&e,"position")?)?;
    let color = parse_color(&childNode!(&e,"farbe")?)?;
    Ok(LightSource { pos, color })
}

fn parse_scene(xml: &str) -> Result<Scene,anyhow::Error> {
    let doc = Document::parse(xml).expect("Parsing XML");
    let e = doc.root_element();
    let triangulation_src = "scene/".to_owned()+&satt(&childNode!(&e,"triangulation")?,"src")?;
    let txml = fs::read_to_string(triangulation_src)?;
    let (materials, triangles) = parse_triangulation(&txml).unwrap();
    let camera = parse_camera(&childNode!(&e,"kamera")?)?;
    let beleuchtung = &childNode!(&e,"beleuchtung")?;
    let background = parse_color(&childNode!(&beleuchtung,"hintergrundfarbe")?)?;
    let ambient = parse_color(
        &childNode!(&beleuchtung,"ambientehelligkeit")?,
    )?;
    Ok(Scene {
        materials,
        triangles,
        background,
        ambient,
        
    })
}
/*<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE szene SYSTEM "szene.dtd">*/
#[test]
fn test_parse_scene() {
    const XML: &str = r#"
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
    let _scene = parse_scene(XML).unwrap();
}
