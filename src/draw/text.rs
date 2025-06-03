use crate::{GlData, State, World};
use mat_vec::Vector3;
use n_body_sim::gl::types::GLvoid;
use n_body_sim::{gl, SIZE_OF_GL_FLOAT};

pub unsafe fn draw_text(gl_res: &GlData, _world: &World, _state: &State, window_size: (i32, i32)) {
    let shader_id = gl_res.get_shader_gl_id("Text shader");
    gl::UseProgram(shader_id);
    let vertex_arr = gl_res.get_vertex_array_gl_id("Position and Texture");
    gl::BindVertexArray(vertex_arr);
    gl::ActiveTexture(gl::TEXTURE0);

    let (_w, _h) = (window_size.0 as f32, window_size.1 as f32);
    gl_res.set_uniform_vec3f("text_color", "Text shader", Vector3::new(0.7, 0.3, 0.1));

    let text = "abc 123 DEF";
    let scale = 1.0;

    let mut text_x = -100; // in pixels
    let text_y = 100;
    for ch in text.chars() {
        if let Some(glyph) = gl_res.get_glyph(ch) {
            let ch_x = (text_x + glyph.bearing.x()) as f32 * scale;
            let ch_y = text_y as f32 + glyph.bearing.y() as f32 * scale;
            let ch_w = glyph.size.x() as f32 * scale;
            let ch_h = glyph.size.y() as f32 * scale;
            let vertices = [
                [ch_x, ch_y + ch_h, 0.0, 0.0],
                [ch_x, ch_y, 0.0, 1.0],
                [ch_x + ch_w, ch_y, 1.0, 1.0],
                [ch_x, ch_y + ch_h, 0.0, 0.0],
                [ch_x + ch_w, ch_y, 1.0, 1.0],
                [ch_x + ch_w, ch_y + ch_h, 1.0, 0.0],
            ];
            let mut vertices_raw = [0.0; 4 * 6];
            for r in 0..6 {
                for j in 0..4 {
                    vertices_raw[r * 4 + j] = vertices[r][j];
                }
            }
            gl::BindTexture(gl::TEXTURE_2D, glyph.texture_id);
            let vertex_buf = gl_res.get_vertex_buffer_gl_id("Quad");
            gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buf);
            gl::BufferSubData(
                gl::ARRAY_BUFFER,
                0,
                vertices_raw.len() as isize * SIZE_OF_GL_FLOAT,
                vertices_raw.as_ptr() as *const GLvoid,
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            text_x += (glyph.advance * scale) as i32;

            gl::DrawArrays(gl::TRIANGLES, 0, 6);
        }
    }
}
