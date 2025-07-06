//use super::select::*;
//use crate::sim::World;
//use crate::update_gui;
use crate::{GlData, State};
use mat_vec::{Matrix4x4, Vector3};
use n_body_sim::gl;
//use n_body_sim::gui::RootGIE;

pub fn view_pos_changed(
    gl_res: &GlData,
    state: &mut State,
    window_size: (i32, i32),
    //world: &World,
    //gui: &mut RootGIE,
) {
    //let last_vec = Vector3::from_tuple(state.last_cursor_pos); //todo: Vector3 from tuple-2 ("as Vector2")
    let (x, y) = state.last_cursor_pos;
    let last_vec = Vector3::new(x, y, 0.0);
    let (px, py) = state.cursor_pos_when_press;
    let press_vec = Vector3::new(px, py, 0.0);
    let (dx, dy, _) = (press_vec - last_vec).get_components();
    let (w, h) = window_size;
    let ratio = (dx as f32 / w as f32, dy as f32 / h as f32);
    state.new_view_pos = state.view_pos + Vector3::new(ratio.0, -ratio.1, 0.0) * state.view_scale;

    let view_mat = Matrix4x4::<f32>::new_LookAt_matrix(
        state.new_view_pos,
        Vector3::new(0.0, 0.0, -1.0),
        Vector3::new(0.0, 1.0, 0.0),
    );
    unsafe {
        let body_shader = gl_res.get_shader_gl_id("Body shader");
        gl::UseProgram(body_shader);
        gl_res.set_uniform_mat4x4("view_mat", "Body shader", &view_mat);

        /*let text_shader = gl_res.get_shader_gl_id("Text shader");
        gl::UseProgram(text_shader);
        let proj_mat = Matrix4x4::<f32>::new_orthographic_projection(
            w as f32, h as f32, 1.0, -0.1, /* fmt force new line */
        );
        gl_res.set_uniform_mat4x4("proj_mat", "Text shader", &proj_mat);*/
    }
    //update_gui(state, world, window_size, gui)
}

pub fn view_scale_changed(gl_res: &GlData, state: &State, window_size: (i32, i32)) {
    let (w, h) = (window_size.0 as f32, window_size.1 as f32);
    let ratio = w / h;
    let proj_mat = Matrix4x4::<f32>::new_orthographic_projection(
        state.view_scale * ratio,
        state.view_scale,
        10.0,
        0.1,
    );
    unsafe {
        let shader = gl_res.get_shader_gl_id("Body shader");
        gl::UseProgram(shader);
        gl_res.set_uniform_mat4x4("proj_mat", "Body shader", &proj_mat);
    }
}

pub fn window_size_changed(gl_res: &GlData, window_size: (i32, i32)) {
    let text_shader = gl_res.get_shader_gl_id("Text shader");
    let (w, h) = window_size;
    unsafe {
        gl::UseProgram(text_shader);
        let proj_mat = Matrix4x4::<f32>::new_orthographic_projection(
            w as f32, h as f32, 1.0, -0.1, /* fmt force new line */
        );
        gl_res.set_uniform_mat4x4("proj_mat", "Text shader", &proj_mat);
    }
}
