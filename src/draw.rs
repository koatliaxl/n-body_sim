use crate::init::draw::TRAJECTORY_DRAW_BUFFER_SIZE as BUF_SIZE;
use crate::state_and_cfg::Config;
use crate::{GlData, State, World};
use mat_vec::{Matrix4x4, Vector3 /*, Vector4*/};
use n_body_sim::{gl, SIZE_OF_GL_FLOAT};

pub use text::*;

pub mod text;

pub static BODY_GFX_SCALE: f32 = 0.4;

pub fn draw(
    gl_res: &GlData,
    world: &World,
    state: &State,
    _window_size: (i32, i32),
    _cfg: &Config,
) {
    unsafe {
        gl::ClearColor(0.2, 0.1, 0.5, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);

        gl::PointSize(3.0);
        //gl::LineWidth(2.0);

        draw_bodies(gl_res, world, state);
        draw_trajectory(gl_res, world, state /*, cfg*/);
    }
}

unsafe fn draw_bodies(gl_res: &GlData, world: &World, state: &State) {
    let shader = gl_res.get_shader_gl_id("body_shader");
    gl::UseProgram(shader);
    let vertex_buf = gl_res.get_vertex_buffer_gl_id("circle");
    gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buf);
    let vertex_arr = gl_res.get_vertex_array_gl_id("only_position");
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
        gl_res.set_uniform_mat4x4("model_mat", "body_shader", &model_mat);
        let color = Vector3::new(1.0, 0.1, 0.5);
        gl_res.set_uniform_vec3f("color", "body_shader", color);
        //gl::DrawArrays(gl::POINTS, 0, 16);
        if obj.get_id() == state.selected as u64 {
            gl::DrawArrays(gl::LINES, 0, 16);
        } else {
            gl::DrawArrays(gl::LINE_LOOP, 0, 16);
        }
    }
}

unsafe fn draw_trajectory(gl_res: &GlData, _world: &World, state: &State /*, cfg: &Config*/) {
    let shader = gl_res.get_shader_gl_id("trajectory_shader");
    gl::UseProgram(shader);
    let vertex_arr = gl_res.get_vertex_array_gl_id("dynamic_only_position");
    //let vertex_arr = gl_res.get_vertex_array_gl_id("only_position");
    gl::BindVertexArray(vertex_arr);

    gl_res.set_uniform_vec3f("color", "trajectory_shader", Vector3::new(0.1, 0.3, 1.0));

    let vertex_buf = gl_res.get_vertex_buffer_gl_id("dynamic-10");
    gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buf);
    let mut trajectory_buf = [0.0_f32; BUF_SIZE];
    let mut s = 0;
    let mut i = 2;
    let mut first_chunk = true;
    while s < state.prediction.trajectory.len() {
        let (x, y, _) = state.prediction.trajectory[s].get_components();
        trajectory_buf[i] = x as f32;
        trajectory_buf[i + 1] = y as f32;
        s += 1;
        i += 2;
        if i >= BUF_SIZE || s == state.prediction.trajectory.len() {
            gl::BufferSubData(
                gl::ARRAY_BUFFER,
                0,
                trajectory_buf.len() as isize * SIZE_OF_GL_FLOAT,
                trajectory_buf.as_ptr() as *const _,
            );
            if !first_chunk {
                gl::DrawArrays(gl::LINE_STRIP, 0, 6);
            } else {
                gl::DrawArrays(gl::LINE_STRIP, 1, 5);
            }
            trajectory_buf[0] = trajectory_buf[BUF_SIZE - 2];
            trajectory_buf[1] = trajectory_buf[BUF_SIZE - 1];
            i = 2;
            first_chunk = false;
        }
    }
    gl::BindBuffer(gl::ARRAY_BUFFER, 0);
}
