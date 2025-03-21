use crate::gl;
use crate::GlData;
use n_body_sim::SIZE_OF_GL_FLOAT;
use std::ffi::c_void;

pub unsafe fn init_draw(gl_data: &mut GlData) {
    /*gl::VertexAttribPointer(
        1,
        color_attrib_len,
        gl::FLOAT,
        gl::FALSE,
        SIZE_OF_GL_FLOAT as i32 * (position_attrib_len + color_attrib_len),
        (SIZE_OF_GL_FLOAT * position_attrib_len as isize) as *const c_void,
    );
    gl::EnableVertexAttribArray(1);*/

    use std::f32::consts::PI;
    let mut vertices = [0.0_f32; 32];
    for ang in 0..16 {
        let sin = (2.0 * PI / 16.0 * ang as f32).sin();
        let cos = (2.0 * PI / 16.0 * ang as f32).cos();
        vertices[ang * 2] = 0.4 * sin;
        vertices[ang * 2 + 1] = 0.4 * cos;
    }
    let position_attrib_len = 2;

    let mut vertex_buf = 0;
    gl::GenBuffers(1, &mut vertex_buf);
    gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buf);
    gl::BufferData(
        gl::ARRAY_BUFFER,
        vertices.len() as isize * SIZE_OF_GL_FLOAT,
        vertices.as_ptr() as *const c_void,
        gl::STATIC_DRAW,
    );

    let mut vertex_array = 0;
    gl::GenVertexArrays(1, &mut vertex_array);
    gl::BindVertexArray(vertex_array);

    gl::VertexAttribPointer(
        0,
        position_attrib_len,
        gl::FLOAT,
        gl::FALSE,
        SIZE_OF_GL_FLOAT as i32 * (position_attrib_len),
        0 as *const c_void,
    );
    gl::EnableVertexAttribArray(0);
    gl_data.add_vertex_array_gl_id("Only Position", vertex_array);
    gl_data.add_vertex_buffer_gl_id("3 Points", vertex_buf)
}
