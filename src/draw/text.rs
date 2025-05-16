use crate::{GlData, State, World};
use mat_vec::{Matrix4x4, Vector3};
use n_body_sim::gl;

pub unsafe fn draw_text(
    gl_res: &GlData,
    _world: &World,
    _state: &State, /*, window_size: (i32, i32)*/
) {
    let shader_id = gl_res.get_shader_gl_id("Text shader");
    gl::UseProgram(shader_id);
    let vertex_buf = gl_res.get_vertex_buffer_gl_id("Quad");
    gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buf);
    let vertex_arr = gl_res.get_vertex_array_gl_id("Position and texture");
    gl::BindVertexArray(vertex_arr);

    //let (w, h) = (window_size.0 as f32, window_size.1 as f32);

    let text_color = Vector3::new(0.7, 0.3, 0.1);
    gl_res.set_uniform_vec3f("text_color", "Text shader", text_color);

    let text = "Abc 123 Def";
    for ch in text.chars() {
        let glyph = gl_res.get_glyph(ch);

    }
}
