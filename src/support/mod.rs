pub mod id_table;

pub fn in_pixels(scr_val: (f32, f32), windows_size: (i32, i32)) -> (i32, i32) {
    (
        ((scr_val.0 + 1.0) * 0.5 * windows_size.0 as f32) as i32,
        ((1.0 - scr_val.1) * 0.5 * windows_size.1 as f32) as i32,
    )
}
