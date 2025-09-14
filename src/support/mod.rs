use mat_vec::Vector3;

pub mod id_table;

pub fn in_pixels(scr_val: (f32, f32), windows_size: (i32, i32)) -> (i32, i32) {
    (
        ((scr_val.0 + 1.0) * 0.5 * windows_size.0 as f32) as i32,
        ((1.0 - scr_val.1) * 0.5 * windows_size.1 as f32) as i32,
    )
}

#[deprecated]
pub fn conv_coord_base(pos_on_scr: Vector3<f32>, window_size: (i32, i32)) -> (i32, i32) {
    let (w, h) = (window_size.0 as f32, window_size.1 as f32);
    (
        (pos_on_scr.x() - w / 2.0) as i32,
        (-pos_on_scr.y() + h / 2.0) as i32,
    )
}
