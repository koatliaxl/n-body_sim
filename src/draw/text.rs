use crate::GlData;
use mat_vec::Vector3;
use n_body_sim::gl::types::GLvoid;
use n_body_sim::{gl, SIZE_OF_GL_FLOAT};

static WHITESPACE_WIDTH: f32 = 10.0;

//todo multi-line
pub unsafe fn draw_text(gl_res: &GlData, text: &str, pos: (i32, i32), scale: f32) {
    let shader_id = gl_res.get_shader_gl_id("text_shader");
    gl::UseProgram(shader_id);
    let vertex_arr = gl_res.get_vertex_array_gl_id("position_and_texture");
    gl::BindVertexArray(vertex_arr);
    gl::ActiveTexture(gl::TEXTURE0);

    gl_res.set_uniform_vec3f("text_color", "text_shader", Vector3::new(0.7, 0.3, 0.1));

    let mut text_x = pos.0; // in pixels
    let text_y = pos.1; // in pixels
    for ch in text.chars() {
        if let Some(glyph) = gl_res.get_glyph(ch) {
            let ch_x = (text_x + glyph.bearing.x()) as f32;
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
            let vertex_buf = gl_res.get_vertex_buffer_gl_id("dynamic-24");
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
        } else if ch == ' ' {
            text_x += (WHITESPACE_WIDTH * scale) as i32;
        }
    }
}
