use crate::{GlData, State, World};
use mat_vec::{Matrix4x4, Vector3};
use n_body_sim::gl;

pub unsafe fn draw_text(gl_res: &GlData, _world: &World, _state: &State, window_size: (i32, i32)) {
    let shader_id = gl_res.get_shader_gl_id("Text shader");
    gl::UseProgram(shader_id);
    let vertex_buf = gl_res.get_vertex_buffer_gl_id("Quad");
    gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buf);
    let vertex_arr = gl_res.get_vertex_array_gl_id("Position and texture");
    gl::BindVertexArray(vertex_arr);

    //let (w, h) = (window_size.0 as f32, window_size.1 as f32);

    //let text_color = Vector3::new(0.7, 0.3, 0.1);
    gl_res.set_uniform_vec3f("text_color", "Text shader", Vector3::new(0.7, 0.3, 0.1));

    let text = "Abc 123 Def";
    let mut pos_x = 40; // in pixels
    let pos_y = 150;
    for ch in text.chars() {
        if let Some(glyph) = gl_res.get_glyph(ch) {
            let rel_w = glyph.size.x() / window_size.0;
            let rel_h = glyph.size.y() / window_size.1;
            let scaling = Matrix4x4::new_scaling(rel_w as f32, rel_h as f32, 0.0);
            let rel_pos_x = (pos_x + glyph.bearing.x()) / window_size.0;
            let rel_pos_y = (pos_y + glyph.bearing.y()) / window_size.1;
            let translation = Matrix4x4::new_translation(rel_pos_x as f32, rel_pos_y as f32, 0.0);
            let pos_mat = translation * scaling;
            gl_res.set_uniform_mat4x4("pos_mat", "Text shader", &pos_mat);
            pos_x += glyph.advance;
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, glyph.texture_id);
            gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 6);
        }
    }
}
