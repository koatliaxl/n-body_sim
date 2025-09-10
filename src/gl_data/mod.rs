use crate::gl;
use mat_vec::{Matrix4x4, Vector3, Vector4};
use std::collections::HashMap;

pub struct GlData {
    shaders: HashMap<String, u32>,
    vex_bufs: HashMap<String, u32>,
    vex_arr: HashMap<String, u32>,
    var_locations: HashMap<String, HashMap<String, i32>>,

    glyphs: Vec<Glyph>,
}

#[derive(Copy, Clone)]
pub struct Glyph {
    pub symbol: char,
    pub texture_id: u32,
    pub size: Vector3<i32>,
    pub bearing: Vector3<i32>,
    pub advance: f32,
}

impl GlData {
    pub fn new() -> GlData {
        GlData {
            shaders: HashMap::new(),
            vex_bufs: HashMap::new(),
            vex_arr: HashMap::new(),
            var_locations: HashMap::new(),

            glyphs: Vec::new(),
        }
    }
    pub fn get_shader_gl_id(&self, name: &str) -> u32 {
        if let Some(id) = self.shaders.get(name) {
            *id
        } else {
            panic!("The is no shader of the name1: {}", name)
        }
    }
    pub fn add_shader_gl_id(&mut self, name: &str, gl_id: u32) {
        self.shaders.insert(name.to_string(), gl_id);
    }

    pub fn get_vertex_buffer_gl_id(&self, name: &str) -> u32 {
        if let Some(id) = self.vex_bufs.get(name) {
            *id
        } else {
            panic!("The is no vertex buffer object of the name: {}", name)
        }
    }

    pub fn add_vertex_buffer_gl_id(&mut self, name: &str, gl_id: u32) {
        self.vex_bufs.insert(name.to_string(), gl_id);
    }

    pub fn get_vertex_array_gl_id(&self, name: &str) -> u32 {
        if let Some(id) = self.vex_arr.get(name) {
            *id
        } else {
            panic!("The is no vertex array object of the name: {}", name)
        }
    }

    pub fn add_vertex_array_gl_id(&mut self, name: &str, gl_id: u32) {
        self.vex_arr.insert(name.to_string(), gl_id);
    }

    pub fn get_variable_location(&self, shader_name: &str, variable_name: &str) -> i32 {
        if let Some(var_locations) = self.var_locations.get(shader_name) {
            if let Some(var_loc) = var_locations.get(variable_name) {
                *var_loc
            } else {
                panic!("The is no variable \"{}\"", variable_name)
            }
        } else {
            panic!("The is no shader of the name: {}", shader_name)
        }
    }

    pub fn add_variable_location(&mut self, shader_name: &str, variable_name: &str, var_loc: i32) {
        if let Some(ref mut var_locations) = self.var_locations.get_mut(shader_name) {
            var_locations.insert(variable_name.to_string(), var_loc);
        } else if self.shaders.contains_key(shader_name) {
            let mut var_locations = HashMap::new();
            var_locations.insert(variable_name.to_string(), var_loc);
            self.var_locations
                .insert(shader_name.to_string(), var_locations);
        } else {
            panic!("The is no shader of the name: {}", shader_name)
        }
    }

    pub fn add_glyph(&mut self, glyph: Glyph) {
        self.glyphs.push(glyph)
    }

    pub fn get_glyph(&self, char: char) -> Option<Glyph> {
        for glyph in &self.glyphs {
            if glyph.symbol == char {
                return Some(*glyph);
            }
        }
        //panic!("There is no glyph for: {}", char)
        None
    }

    #[allow(dead_code)]
    pub unsafe fn set_uniform_vec3f(&self, name: &str, shader_program: &str, vec: Vector3<f32>) {
        let var_location = self.get_variable_location(shader_program, name);
        let (v1, v2, v3) = vec.get_components();
        gl::Uniform3f(var_location, v1, v2, v3);
    }

    pub unsafe fn set_uniform_vec4f(&self, name: &str, shader_program: &str, vec: Vector4<f32>) {
        let var_location = self.get_variable_location(shader_program, name);
        let (v1, v2, v3, v4) = vec.get_components();
        gl::Uniform4f(var_location, v1, v2, v3, v4);
    }

    pub unsafe fn set_uniform_mat4x4(
        &self,
        name: &str,
        shader_program: &str,
        matrix: &Matrix4x4<f32>,
    ) {
        let var_location = self.get_variable_location(shader_program, name);
        gl::UniformMatrix4fv(var_location, 1, gl::TRUE, matrix.as_ptr());
    }
}
