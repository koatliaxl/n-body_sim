pub mod draw;
pub mod gui;
pub mod shader;
pub mod text;
pub mod world;

pub use draw::init_draw;
pub use gui::*;
pub use text::*;

use super::gl;
#[allow(unused_imports)]
use gl::{FRAGMENT_SHADER, GEOMETRY_SHADER, VERTEX_SHADER};
//use glutin::window::Window;
use crate::GlData;
use glfw::{
    fail_on_errors, Callback, Context, Error, Glfw, OpenGlProfileHint, Version, Window,
    WindowEvent, WindowHint,
};
use shader::*;
use std::sync::mpsc::Receiver;

pub fn init_glfw() -> (Glfw, Window, Receiver<(f64, WindowEvent)>) {
    let Version {
        major,
        minor,
        patch,
    } = glfw::get_version();
    println!("GLFW run-time version: {}.{}.{}", major, minor, patch);
    let error_callback = Some(Callback {
        f: fail_on_errors as fn(Error, String, &()),
        data: (),
    });
    let mut glfw = glfw::init(error_callback).unwrap();
    glfw.window_hint(WindowHint::ContextVersionMajor(3));
    glfw.window_hint(WindowHint::ContextVersionMinor(3));
    glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));
    //glfw.window_hint(WindowHint::Samples(Some(4)));
    let (mut window, events) = glfw
        .create_window(400, 300, "OpenGL learn", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window");
    window.make_current();
    window.set_key_polling(true);
    window.set_cursor_pos_polling(true);
    window.set_cursor_enter_polling(true);
    //window.set_focus_polling(true);
    window.set_scroll_polling(true);
    window.set_mouse_button_polling(true);
    window.set_size_polling(true);
    //window.set_cursor_mode(CursorMode::Disabled);
    /*window.set_cursor_pos(
        window.get_size().0 as f64 / 2.0,
        window.get_size().1 as f64 / 2.0,
    );*/
    (glfw, window, events)
}

pub fn init_open_gl(window_size: (i32, i32)) {
    unsafe {
        gl::Viewport(0, 0, window_size.0, window_size.1);
        println!("Viewport is loaded: {}", gl::Viewport::is_loaded());
        let parameters = [
            (gl::MAX_VERTEX_ATTRIBS, "Max vertex attributes supported"),
            (
                gl::MAX_VERTEX_UNIFORM_COMPONENTS,
                "Max vertex uniform components supported",
            ),
            (
                gl::MAX_FRAGMENT_UNIFORM_COMPONENTS,
                "Max fragment uniform components supported",
            ),
        ];
        for (param, msg) in &parameters {
            let mut value = 0;
            gl::GetIntegerv(*param, &mut value);
            println!("{}: {}", msg, value);
        }
    }
    println!()
}

pub fn init_shaders(gl_data: &mut GlData) {
    unsafe {
        let obj_vertex = gen_shader_from_file(
            "shaders/obj_vertex.glsl",
            VERTEX_SHADER,
            "Object vertex shader",
        );
        /*let obj_geom = gen_shader_from_file(
            "shaders/obj_geom.glsl",
            GEOMETRY_SHADER,
            "Object geometry shader",
        );*/
        let obj_frag = gen_shader_from_file(
            "shaders/obj_frag.glsl",
            FRAGMENT_SHADER,
            "Object fragment shader",
        );
        let obj_shader = gen_shader_program(obj_vertex, obj_frag, "Object shader");
        /*let shd_program =
        gen_geometry_shader_program(obj_vertex, obj_geom, obj_frag, "Object shader");*/
        gl_data.add_shader_gl_id("Object shader", obj_shader);
        let view_mat = gl::GetUniformLocation(obj_shader, "view_mat\0".as_ptr() as *const i8);
        let model_mat = gl::GetUniformLocation(obj_shader, "model_mat\0".as_ptr() as *const i8);
        let proj_mat = gl::GetUniformLocation(obj_shader, "proj_mat\0".as_ptr() as *const i8);
        gl_data.add_variable_location("Object shader", "view_mat", view_mat);
        gl_data.add_variable_location("Object shader", "model_mat", model_mat);
        gl_data.add_variable_location("Object shader", "proj_mat", proj_mat);

        let text_vertex = gen_shader_from_file(
            "shaders/text_vertex.glsl",
            VERTEX_SHADER,
            "Text vertex shader",
        );
        let text_frag = gen_shader_from_file(
            "shaders/text_frag.glsl",
            FRAGMENT_SHADER,
            "Text fragment shader",
        );
        let text_shader = gen_shader_program(text_vertex, text_frag, "Text shader");
        let proj_mat = gl::GetUniformLocation(text_shader, "proj_mat\0".as_ptr() as *const i8);
        let glyph = gl::GetUniformLocation(text_shader, "glyph\0".as_ptr() as *const i8);
        let text_color = gl::GetUniformLocation(text_shader, "text_color\0".as_ptr() as *const i8);
        let pos_mat = gl::GetUniformLocation(text_shader, "pos_mat\0".as_ptr() as *const i8);
        gl_data.add_shader_gl_id("Text shader", text_shader);
        gl_data.add_variable_location("Text shader", "proj_mat", proj_mat);
        gl_data.add_variable_location("Text shader", "glyph", glyph);
        gl_data.add_variable_location("Text shader", "text_color", text_color);
        gl_data.add_variable_location("Text shader", "pos_mat", pos_mat);

        gl::DeleteShader(obj_vertex);
        //gl::DeleteShader(obj_geom);
        gl::DeleteShader(obj_frag);
        gl::DeleteShader(text_vertex);
        gl::DeleteShader(text_frag);
    }
}
