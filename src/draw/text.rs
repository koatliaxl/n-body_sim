use crate::{GlData, State, World};
use mat_vec::{Matrix4x4, Vector3};
use n_body_sim::gl;

pub unsafe fn draw_text(gl_res: &GlData, _world: &World, _state: &State, window_size: (i32, i32)) {
    let shader_id = gl_res.get_shader_gl_id("Text shader");
    gl::UseProgram(shader_id);
    /*let vertex_buf = gl_res.get_vertex_buffer_gl_id("Circle");
    gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buf);*/
    let vertex_arr = gl_res.get_vertex_array_gl_id("Position and Texture");
    gl::BindVertexArray(vertex_arr);
    gl::ActiveTexture(gl::TEXTURE0);

    let (w, h) = (window_size.0 as f32, window_size.1 as f32);

    //let text_color = Vector3::new(0.7, 0.3, 0.1);
    gl_res.set_uniform_vec3f("text_color", "Text shader", Vector3::new(0.7, 0.3, 0.1));

    let text = "A";
    let mut pos_x = 10; // in pixels
    let pos_y = 50;
    for ch in text.chars() {
        if let Some(glyph) = gl_res.get_glyph(ch) {
            let rel_w = glyph.size.x() as f32 / w;
            let rel_h = glyph.size.y() as f32 / h;
            let scaling = Matrix4x4::new_scaling(rel_w, rel_h, 1.0);
            let rel_pos_x = (pos_x + glyph.bearing.x()) as f32 / w;
            let rel_pos_y = (pos_y + glyph.bearing.y()) as f32 / h;
            let translation = Matrix4x4::new_translation(rel_pos_x, rel_pos_y, 0.0);
            let pos_mat = translation * scaling;
            //println!("{}, {}; {}, {}", rel_w, rel_h, rel_pos_x, rel_pos_y);
            gl_res.set_uniform_mat4x4("pos_mat", "Text shader", &pos_mat);
            pos_x += (glyph.advance as f32/*/ 26.6*/) as i32;
            //println!("{}", pos_x);
            gl::BindTexture(gl::TEXTURE_2D, glyph.texture_id);
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
        }
    }
}
