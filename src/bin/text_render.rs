use freetype as ft;

const WIDTH: i32 = 64;
const HEIGHT: i32 = 48;

type Figure = [[u8; WIDTH as usize]; HEIGHT as usize];

fn main() {
    let library = ft::Library::init().unwrap();
    let face = library.new_face("assets/Lexend-Regular.ttf", 0).unwrap();

    face.set_char_size(40 * 64, 0, 50, 0).unwrap();
    face.load_char('q' as usize, ft::face::LoadFlag::RENDER)
        .unwrap();

    let glyph = face.glyph();
    let x = glyph.bitmap_left() + 5;
    let y = HEIGHT - glyph.bitmap_top() - 10;

    let figure = draw_bitmap(glyph.bitmap(), x, y);
    let mapping = b" .:-;+*x#@";
    let mapping_scale = (mapping.len() - 1) as f32;
    for row in figure {
        for v in row {
            let i = ((v as f32 / 256.0) * mapping_scale + 0.5) as usize;
            // '$' in the output if something wrong
            let c = mapping.get(i).cloned().unwrap_or(b'$') as char;
            print!("{}", c)
        }
        println!(" ")
    }
}

fn draw_bitmap(bitmap: ft::Bitmap, x: i32, y: i32) -> Figure {
    let mut figure = [[0; WIDTH as usize]; HEIGHT as usize];
    let w = (bitmap.width() * 2) as usize;
    let x_max = x + w as i32;
    let y_max = y + bitmap.rows();
    for (p, i) in (x..x_max).enumerate() {
        for (q, j) in (y..y_max).enumerate() {
            if i < 0 || j < 0 || i >= WIDTH || j >= HEIGHT {
                continue;
            }
            figure[j as usize][i as usize] |= bitmap.buffer()[(q * w + p) / 2];
        }
    }
    figure
}
