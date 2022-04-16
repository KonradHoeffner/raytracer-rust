#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(unused_imports)]
#![allow(clippy::single_match)]
#![allow(clippy::zero_ptr)]
// https://raw.githubusercontent.com/rust-tutorials/learn-opengl/main/examples/002-triangle-arrays2.rs

mod bounding_box;
mod camera;
mod color;
mod file_import;
mod learn;
mod light_source;
mod material;
mod scene;
mod triangle;

use crate::file_import::parse_scene;
use std::fs;

use crate::learn::*;
use beryllium::*;

const WINDOW_TITLE: &str = "Raytracer";

use beryllium::*;
use core::{
    convert::{TryFrom, TryInto},
    mem::{size_of, size_of_val},
};
use ogl33::*;
use glam::f32::Mat4;

type Vertex = [f32; 3];

//const VERTICES: [Vertex; 3] = [[-0.5, -0.5, 0.0], [0.5, -0.5, 0.0], [0.0, 0.5, 0.0]];

const VERT_SHADER: &str = r#"#version 330 core
  uniform mat4 transform;

  layout (location = 0) in vec3 pos;

  void main() {
    gl_Position = transform * vec4(pos,1.0);
  }
"#;

const FRAG_SHADER: &str = r#"#version 330 core
  out vec4 final_color;

  void main() {
    final_color = vec4(1.0, 0.5, 0.2, 1.0);
  }
"#;

fn main() {
    let xml = fs::read_to_string("scene/abgabeszene.xml").unwrap();
    let scene = parse_scene(&xml).unwrap();
    let mut vertices: Vec<f32> = Vec::new();
    // todo: use flat map
    for t in scene.triangles {
        for p in t.p {
            vertices.push(p.x);
            vertices.push(-p.y);
            vertices.push(p.z);
        }
    }
    //vertices.extend([-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0].iter().cloned()); // test

    let sdl = SDL::init(InitFlags::Everything).expect("couldn't start SDL");
    sdl.gl_set_attribute(SdlGlAttr::MajorVersion, 3).unwrap();
    sdl.gl_set_attribute(SdlGlAttr::MinorVersion, 3).unwrap();
    sdl.gl_set_attribute(SdlGlAttr::Profile, GlProfile::Core)
        .unwrap();
    #[cfg(target_os = "macos")]
    {
        sdl.gl_set_attribute(SdlGlAttr::Flags, ContextFlag::ForwardCompatible)
            .unwrap();
    }

    let win = sdl
        .create_gl_window(
            WINDOW_TITLE,
            WindowPosition::Centered,
            800,
            600,
            WindowFlags::Shown,
        )
        .expect("couldn't make a window and context");
    win.set_swap_interval(SwapInterval::Vsync);

    unsafe {
        load_gl_with(|f_name| win.get_proc_address(f_name));
    }

    clear_color(0.2, 0.3, 0.3, 1.0);

    let vao = VertexArray::new().expect("Couldn't make a VAO");
    vao.bind();

    let vbo = Buffer::new().expect("Couldn't make a VBO");
    vbo.bind(BufferType::Array);

    buffer_data(
        BufferType::Array,
        bytemuck::cast_slice(&vertices),
        GL_STATIC_DRAW,
    );

    unsafe {
        glVertexAttribPointer(
            0,
            3,
            GL_FLOAT,
            GL_FALSE,
            size_of::<Vertex>().try_into().unwrap(),
            0 as *const _,
        );
        glEnableVertexAttribArray(0);
    }

    let shader_program = ShaderProgram::from_vert_frag(VERT_SHADER, FRAG_SHADER).unwrap();
    shader_program.use_program();

    'main_loop: loop {
        // handle events this frame
        while let Some(event) = sdl.poll_events().and_then(Result::ok) {
            match event {
                Event::Quit(_) => break 'main_loop,
                _ => (),
            }
        }
        //let time = sdl.get_ticks() as f32 / 2000.0_f32;
        //let transform = Mat4::from_rotation_z(time);
        let transform = Mat4::perspective_lh(1.0,16.0/9.0,scene.camera.clippingNear,scene.camera.clippingFar);
        // and then draw!
        unsafe {
            glClear(GL_COLOR_BUFFER_BIT);
            let transform_name = null_str!("transform").as_ptr().cast();
            let transform_loc = glGetUniformLocation(shader_program.0, transform_name);
            glUniformMatrix4fv(transform_loc, 1, GL_FALSE, &transform.to_cols_array()[0]);
            glDrawArrays(GL_TRIANGLES, 0, (3 * vertices.len()).try_into().unwrap());
        }
        win.swap_window();
    }
}
