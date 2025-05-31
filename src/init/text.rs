use crate::{gl, GlData, Glyph};
use freetype as ft;
use freetype::face::LoadFlag;
use mat_vec::Vector3;
use std::ffi::c_void;

pub fn init_glyphs(gl_data: &mut GlData) {
    let library = ft::Library::init().expect("Failed to initialize FreeType");
    let face = library
        .new_face("assets/Lexend-Regular.ttf", 0)
        .expect("Failed to load font");
    face.set_pixel_sizes(0, 48)
        .expect("Failed to set font size");
    unsafe { gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1) };
    for ch in '!'..'~' {
        if let Err(err) = face.load_char(ch as usize, LoadFlag::RENDER) {
            eprintln!("Failed to load glyph ({})", err)
        }
        let mut texture_id = 0;
        unsafe {
            gl::GenTextures(1, &mut texture_id);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RED as i32,
                face.glyph().bitmap().width(),
                face.glyph().bitmap().rows(),
                0,
                gl::RED,
                gl::UNSIGNED_BYTE,
                face.glyph().bitmap().buffer().as_ptr() as *const c_void,
            );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        }
        let size = Vector3::new(
            face.glyph().bitmap().width(),
            face.glyph().bitmap().rows(),
            0,
        );
        let bearing = Vector3::new(face.glyph().bitmap_left(), face.glyph().bitmap_top(), 0);
        let glyph = Glyph {
            symbol: ch,
            texture_id,
            size,
            bearing,
            advance: face.glyph().advance().x,
        };
        gl_data.add_glyph(glyph)
    }
    unsafe {
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        /*use freetype::freetype_sys::FT_Done_Face;
        use freetype::freetype_sys::FT_Done_Library;
        FT_Done_Face(face);
        FT_Done_Library(library)*/
    }
}
