use crate::draw::BODY_GFX_SCALE;
use crate::{State, World};
use mat_vec::{Matrix4x4, Vector3, Vector4};
use n_body_sim::gui::{Label, RootGIE /*, GIE*/};
use n_body_sim::Body;

pub fn select_obj(state: &mut State, world: &World, window_size: (i32, i32)) {
    let (x, y) = (
        state.last_cursor_pos.0 as f32,
        state.last_cursor_pos.1 as f32,
    );
    let (w, h) = (window_size.0 as f32, window_size.1 as f32);
    let (proj_mat, view_mat) = calc_matrices(state, window_size);

    for body in world
        .bodies
        .lock()
        .expect("Main: failed to acquire lock for selection")
        .iter()
    {
        let pos_on_scr = calc_body_pos_on_screen(window_size, &view_mat, &proj_mat, body);
        let equation_val = (x - pos_on_scr.x()).powi(2) + (y - pos_on_scr.y()).powi(2);
        let radius = BODY_GFX_SCALE * body.get_radius() as f32;
        if equation_val < (radius / state.view_scale * (w + h) / 2.0).powi(2) {
            state.selected = body.get_id() as i64;
            state.update_ui_requested = true;
            state.prediction.devalidate_history();
        }
    }
}

pub fn calc_matrices(state: &State, window_size: (i32, i32)) -> (Matrix4x4<f32>, Matrix4x4<f32>) {
    let (w, h) = (window_size.0 as f32, window_size.1 as f32);
    let ratio = w / h;
    let proj_mat = Matrix4x4::<f32>::new_orthographic_projection(
        state.view_scale * ratio,
        state.view_scale,
        10.0,
        0.1,
    );
    let view_mat = Matrix4x4::<f32>::new_LookAt_matrix(
        state.new_view_pos,
        Vector3::new(0.0, 0.0, -1.0),
        Vector3::new(0.0, 1.0, 0.0),
    );
    (proj_mat, view_mat)
}

pub fn calc_body_pos_on_screen(
    window_size: (i32, i32),
    view_mat: &Matrix4x4<f32>,
    proj_mat: &Matrix4x4<f32>,
    body: &Body,
) -> Vector3<f32> {
    let (w, h) = (window_size.0 as f32, window_size.1 as f32);
    let pos_vec4 = Vector4::from(&Vector3::<f32>::from(body.pos));
    let mut pos_on_scr = proj_mat.clone() * view_mat.clone() * pos_vec4;
    pos_on_scr.set_x((pos_on_scr.x() + 1.0) * 0.5 * w);
    pos_on_scr.set_y((1.0 - pos_on_scr.y()) * 0.5 * h);
    pos_on_scr.into()
}

pub fn fix_screen_pos(pos_on_scr: Vector3<f32>, window_size: (i32, i32)) -> (i32, i32) {
    let (w, h) = (window_size.0 as f32, window_size.1 as f32);
    (
        (pos_on_scr.x() - w / 2.0) as i32,
        (-pos_on_scr.y() + h / 2.0) as i32,
    )
}

pub fn update_selected_info(gui: &mut RootGIE, mut pos_on_scr: (i32, i32), body: &Body) {
    let pos_label = gui.get_gie("body_pos_text").unwrap();
    pos_label.change_pos(pos_on_scr);
    pos_label
        .get_type()
        .downcast_mut::<Label>()
        .expect("failed to downcast GIE as Label")
        .change_text(format!("{:.2}, {:.2}", body.pos.x(), body.pos.y()));
    pos_label.get_base_mut().visible = true;
    let mass_label = gui.get_gie("body_mass_text").unwrap();
    pos_on_scr.1 -= 15;
    mass_label.change_pos(pos_on_scr);
    mass_label
        .get_type()
        .downcast_mut::<Label>()
        .expect("failed to downcast GIE as Label")
        .change_text(format!("mass: {}", body.mass));
    mass_label.get_base_mut().visible = true;
}
