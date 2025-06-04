use fontdue::{Font, FontSettings};
use std::io::stdin;

static ASKII_ASPECT_RATIO: usize = 2; // to compensate width to height ratio of console characters
static ASKII_BASE_SIZE: f32 = 10.0;

fn main() {
    let font_data = include_bytes!("../../assets/Lexend-Regular.ttf") as &[u8];
    let font =
        Font::from_bytes(font_data, FontSettings::default()).expect("Failed to construct a font");

    let mut size = 1;
    let mut char = ' ';

    println!("Enter symbol to draw and/or resolution multiplier:");
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("Error while trying read input");
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
                char = ch;
            }
            '\n' | '\r' => (),
            _ => println!("Please, enter a printable character"),
        }
    }
    let (mtr, bitmap) = font.rasterize(char, ASKII_BASE_SIZE * size as f32);
    let mapping = b" .:-;+*x#@";
    let mapping_scale = (mapping.len() - 1) as f32;
    let w = mtr.width * ASKII_ASPECT_RATIO;

    for i in 0..mtr.height {
        for j in 0..w {
            let v = bitmap[(i * w + j) / ASKII_ASPECT_RATIO];
            let i = ((v as f32 / 256.0) * mapping_scale + 0.5) as usize;
            // '$' in the output if something wrong
            let c = mapping.get(i).cloned().unwrap_or(b'$') as char;
            print!("{}", c)
        }
        println!()
    }
}
