use freetype as ft;

//const WIDTH: i32 = 48 * 1;
const HEIGHT: i32 = 32 * 1;

//type Figure = [[u8; WIDTH as usize]; HEIGHT as usize];

fn main() {
    let library = ft::Library::init().unwrap();
    let face = library.new_face("assets/Lexend-Regular.ttf", 0).unwrap();

    println!("Enter symbol to draw and/or size multiplier:");
    let mut input = String::new();
    std::io::stdin()
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
                        /*let b = second.as_bytes();
                        for n in b {
                            print!("{} ", n)
                        }*/
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
            //'\n' => println!("empty input"),
            _ => println!("Please, enter a printable character"),
        }
    } else {
        //println!("empty input")
    }

    let glyph = face.glyph();
    let x = glyph.bitmap_left() + 5;
    let y = HEIGHT - glyph.bitmap_top() - 8 * size as i32;

    let figure = draw_bitmap(glyph.bitmap(), x, y);
    let mapping = b" .:-;+*x#@";
    let mapping_scale = (mapping.len() - 1) as f32;
    for col in 0..figure[0].len() {
        for j in 0..figure.len() {
            let v = figure[j][col];
            let i = ((v as f32 / 256.0) * mapping_scale + 0.5) as usize;
            // '$' in the output if something wrong
            let c = mapping.get(i).cloned().unwrap_or(b'$') as char;
            print!("{}", c)
        }
        println!(/*"x is: {} ", x*/)
    }
}

fn draw_bitmap(bitmap: ft::Bitmap, x: i32, y: i32) -> Vec<Vec<u8>> {
    //let mut figure = [[0; WIDTH as usize]; HEIGHT as usize];
    let mut figure = Vec::new();
    let w = (bitmap.width() * 2) as usize;
    let x_max = x + w as i32;
    let y_max = y + bitmap.rows();
    for (p, _i) in (x..x_max).enumerate() {
        let mut row = Vec::new();
        for (q, _j) in (y..y_max).enumerate() {
            /*if i < 0 || j < 0 || i >= WIDTH || j >= HEIGHT {
                continue;
            }*/
            let v = bitmap.buffer()[(q * w + p) / 2];
            row.push(v);
            //figure[j as usize][i as usize]
        }
        figure.push(row)
    }
    figure
}
