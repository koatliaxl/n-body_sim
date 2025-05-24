use crate::{gl, GlData, Glyph};
/*use freetype as ft;
use freetype::face::LoadFlag;*/
use mat_vec::Vector3;
//use rusttype::{point, Font, Scale};
use std::ffi::c_void;

pub fn init_glyphs(gl_data: &mut GlData) {
    /*let font_data = include_bytes!("../../assets/Lexend-Regular.ttf");
    let font = Font::try_from_bytes(font_data as &[u8]).expect("Failed to construct a font");
    let scale = Scale::uniform(128.0);
    let mut chars = String::new();

    let glyphs: Vec<_> = font
        .layout(
            chars.as_str(),
            scale,
            point(0.0, font.v_metrics(scale).ascent),
        )
        .collect();
    let width = glyphs
        .iter()
        .rev()
        .map(|g| g.position().x as f32 + g.unpositioned().h_metrics().advance_width)
        .next()
        .unwrap_or(0.0)
        .ceil() as usize;
    let height = scale.y as usize;
    let mut i = '!' as u8;*/
    /*let font_data = std::fs::read("assets/Lexend-Regular.ttf").expect("Could not load font");
    let face = ttf_parser::Face::parse(&font_data, 0).expect("Could not parse font file");*/

    let font_data = include_bytes!("../../assets/Lexend-Regular.ttf") as &[u8];
    let font = fontdue::Font::from_bytes(font_data, fontdue::FontSettings::default())
        .expect("Failed to construct a font");

    for ch in '!'..'~' {
        /*let id = face.glyph_index(ch).expect("Could not get glyph");
        let ras = face
            .glyph_raster_image(id, 10)
            .expect("could not rasterize glyph");*/
        let (mtr, bitmap) = font.rasterize(ch, 60.0);

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
        let bearing = Vector3::new(mtr.width as i32 + 20, mtr.height as i32 + 20, 0);

        let glyph = Glyph {
            symbol: ch,
            texture_id,
            size,
            bearing,
            advance: mtr.advance_width,
        };
        gl_data.add_glyph(glyph)
    }
    /*for glyph in glyphs {
        let mut buf = vec![0.0; width*height];
        if let Some(_bounding_box) = glyph.pixel_bounding_box() {
            glyph.draw(|x, y, v| {
                buf[x as usize + y as usize * width] = v;
            });

        }
        println!("{2:}, {} - {}", glyph.id().0, i, i as char);
        i += 1;
    }*/
    unsafe {
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }
}
