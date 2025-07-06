use crate::{GlData, State, World};
use mat_vec::{Matrix4x4, Vector3};
use n_body_sim::gl;

pub use text::*;

pub mod text;

pub static BODY_GFX_SCALE: f32 = 0.4;

pub fn draw(gl_res: &GlData, world: &World, state: &State, _window_size: (i32, i32)) {
    unsafe {
        gl::ClearColor(0.2, 0.1, 0.5, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);

        gl::PointSize(3.0);
        //gl::LineWidth(2.0);

        draw_bodies(gl_res, world, state);
    }
}

unsafe fn draw_bodies(gl_res: &GlData, world: &World, state: &State) {
    let shader = gl_res.get_shader_gl_id("Body shader");
    gl::UseProgram(shader);
    let vertex_buf = gl_res.get_vertex_buffer_gl_id("Circle");
    gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buf);
    let vertex_arr = gl_res.get_vertex_array_gl_id("Only Position");
    gl::BindVertexArray(vertex_arr);

    for obj in world
        .bodies
        .lock()
        .expect("Main: failed to acquire lock for drawing")
        .iter()
    {
        let (x, y, _) = obj.pos.get_components(); //no conversions yet
        let pos = Vector3::new(x as f32, y as f32, 0.0);
        let model_mat = Matrix4x4::new_translation_from_vec(pos);
        let scaling = Matrix4x4::new_uniform_scaling(obj.get_radius() as f32);
        let model_mat = model_mat * scaling;
        gl_res.set_uniform_mat4x4("model_mat", "Body shader", &model_mat);
        //gl::DrawArrays(gl::POINTS, 0, 16);
        if obj.get_id() == state.selected as u64 {
            gl::DrawArrays(gl::LINES, 0, 16);
        } else {
            gl::DrawArrays(gl::LINE_LOOP, 0, 16);
        }
    }
}
