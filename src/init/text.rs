use crate::{gl, GlData, Glyph};
/*use freetype as ft;
use freetype::face::LoadFlag;*/
use mat_vec::Vector3;
use rusttype::{point, Font, Scale};
use std::ffi::c_void;

pub fn init_glyphs(gl_data: &mut GlData) {
    //let library = ft::Library::init().expect("Failed to initialize FreeType");
    /*let face = library
        .new_face("assets/Lexend-Regular.ttf", 0)
        .expect("Failed to load font");
    face.set_pixel_sizes(0, 48)
        .expect("Failed to set font size");*/
    let font_data = include_bytes!("../../assets/Lexend-Regular.ttf");
    let font = Font::try_from_bytes(font_data as &[u8]).expect("Failed to construct a font");
    let scale = Scale::uniform(128.0);
    let mut chars = String::new();
    for ch in '!'..'~' {
        /*if let Err(err) = face.load_char(ch as usize, LoadFlag::RENDER) {
            eprintln!("Failed to load glyph ({})", err)
        }*/
        chars.push(ch)
    }
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
    /*for i in 0..width * height {
        buf.push(0.0)
    }*/
    //println!("{}", buf.len());
    let mut i = '!' as u8;
    for glyph in glyphs {
        let mut buf = vec![0.0; width*height];
        if let Some(bounding_box) = glyph.pixel_bounding_box() {
            /*let mut y_max = 0;
            let mut x_max = 0;*/
            glyph.draw(|x, y, v| {
                /*if y > y_max {
                    y_max = y
                }
                if x > x_max {
                    x_max = x
                }*/
                buf[x as usize + y as usize * width] = v;
            });
            let mut texture_id = 0;
            unsafe {
                gl::GenTextures(1, &mut texture_id);
                gl::BindTexture(gl::TEXTURE_2D, texture_id);
                gl::TexImage2D(
                    gl::TEXTURE_2D,
                    0,
                    gl::RED as i32,
                    width as i32,
                    scale.y as i32,
                    0,
                    gl::RED,
                    gl::UNSIGNED_BYTE,
                    buf.as_ptr() as *const c_void,
                );
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            }
            let size = Vector3::new(width as i32, height as i32, 0);
            let bearing = Vector3::new(height as i32 + 20, height as i32 + 20, 0);
            let glyph = Glyph {
                symbol: i as char,
                texture_id,
                size,
                bearing,
                advance: (20) as i32,
            };
            gl_data.add_glyph(glyph)
        }
        println!("{2:}, {} - {}", glyph.id().0, i, i as char);
        //buf.clear();
        i += 1;
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
