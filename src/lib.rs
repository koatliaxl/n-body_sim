pub const SIZE_OF_GL_FLOAT: isize = std::mem::size_of::<gl::types::GLfloat>() as isize;

pub mod gl {
    include!("../gl/bindings.rs");
}
