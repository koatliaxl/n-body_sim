use crate::{gl, GlData, Glyph};
use mat_vec::Vector3;
use std::ffi::c_void;

pub fn init_glyphs(gl_data: &mut GlData) {
    let font_data = include_bytes!("../../assets/Lexend-Regular.ttf") as &[u8];
    let font = fontdue::Font::from_bytes(font_data, fontdue::FontSettings::default())
        .expect("Failed to construct a font");

    unsafe { gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1); }
    for ch in '!'..'~' {
        let (mtr, bitmap) = font.rasterize(ch, 30.0);

        let mut texture_id = 0;
        unsafe {
            gl::GenTextures(1, &mut texture_id);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RED as i32,
                mtr.width as i32,
                mtr.height as i32,
                0,
                gl::RED,
                gl::UNSIGNED_BYTE,
                bitmap.as_ptr() as *const c_void,
            );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        }
        let size = Vector3::new(mtr.width as i32, mtr.height as i32, 0);
        let bearing = Vector3::new(mtr.xmin, mtr.ymin, 0);
        println!("{}, {}; {}", mtr.xmin, mtr.ymin, mtr.advance_width);
        let glyph = Glyph {
            symbol: ch,
            texture_id,
            size,
            bearing,
            advance: mtr.advance_width,
        };
        gl_data.add_glyph(glyph)
    }
    unsafe {
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }
}
