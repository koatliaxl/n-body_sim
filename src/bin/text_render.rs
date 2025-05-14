use freetype as ft;
use std::io::stdin;

//const WIDTH: i32 = 48 * 1;
//const HEIGHT: i32 = 32 * 1;

fn main() {
    let library = ft::Library::init().unwrap();
    let face = library.new_face("assets/Lexend-Regular.ttf", 0).unwrap();

    println!("Enter symbol to draw and/or size multiplier:");
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("Error while trying read input");
    let mut size = 1;
    let mut chars = input.chars();

    if let Some(ch) = chars.next() {
        match ch {
            '!'..='~' => {
                match chars.next() {
                    Some(' ') => {
                        // split at 2 and not at 1 to omit preceding whitespace
                        let (_, second) = input.split_at(2);
                        if let Some(s) = second.strip_suffix("\r\n") {
                            if let Ok(multiplier) = s.parse::<u32>() {
                                size = multiplier;
                            } else {
                                print!("{}{0:}", s);
                                println!("Size must be a number")
                            }
                        }
                    }
                    Some('\r') | Some('\n') => (),
                    _ => println!("Please, enter just a single character"),
                }
                face.set_char_size(40 * 64, 0, 50 * size, 0).unwrap();
                face.load_char(ch as usize, ft::face::LoadFlag::RENDER)
                    .unwrap();
            }
            '\n' | '\r' => (),
            _ => println!("Please, enter a printable character"),
        }
    }

    let glyph = face.glyph();
    /*let x = glyph.bitmap_left() + 5;
    let y = HEIGHT - glyph.bitmap_top() - 8 * size as i32;*/

    let figure = draw_bitmap(glyph.bitmap() /*, x, y*/);
    let mapping = b" .:-;+*x#@";
    let mapping_scale = (mapping.len() - 1) as f32;
    for i in 0..figure.len() {
        for j in 0..figure[i].len() {
            let v = figure[i][j];
            let i = ((v as f32 / 256.0) * mapping_scale + 0.5) as usize;
            // '$' in the output if something wrong
            let c = mapping.get(i).cloned().unwrap_or(b'$') as char;
            print!("{}", c)
        }
        println!()
    }
}

fn draw_bitmap(bitmap: ft::Bitmap /*, _x: i32, _y: i32*/) -> Vec<Vec<u8>> {
    let mut figure = Vec::new();
    let w = (bitmap.width() * 2) as usize;
    //let x_max = w;
    //let y_max = bitmap.rows() as usize;
    for i in 0..bitmap.rows() as usize {
        let mut row = Vec::new();
        for j in 0..w {
            let v = bitmap.buffer()[(i * w + j) / 2];
            row.push(v);
        }
        figure.push(row)
    }
    figure
}
