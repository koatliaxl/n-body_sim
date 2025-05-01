use crate::draw::BODY_GFX_SCALE;
use crate::{State, World};
use mat_vec::{Matrix4x4, Vector3, Vector4};

pub fn select_obj(state: &mut State, world: &World, window_size: (i32, i32)) {
    state.selected = -1;
    let (x, y) = (
        state.last_cursor_pos.0 as f32,
        state.last_cursor_pos.1 as f32,
    );
    let view_mat = Matrix4x4::<f32>::new_LookAt_matrix(
        state.view_pos,
        Vector3::new(0.0, 0.0, -1.0),
        Vector3::new(0.0, 1.0, 0.0),
    );
    let (w, h) = (window_size.0 as f32, window_size.1 as f32);
    let ratio = w / h;
    let proj_mat = Matrix4x4::<f32>::new_orthographic_projection(
        state.view_scale * ratio,
        state.view_scale,
        10.0,
        0.1,
    );
    for o in world
        .bodies
        .lock()
        .expect("Main: failed to acquire lock for selection")
        .iter()
    {
        let (ox, oy, _) = o.pos.get_components();
        let pos_vec4 = Vector4::new(ox as f32, oy as f32, 0.0, 1.0);
        //todo Vec4: new from Vec3
        //let pos_vec4 = Vector4::from(o.pos);
        let mut pos_on_scr = proj_mat.clone() * view_mat.clone() * pos_vec4;
        pos_on_scr.set_x((pos_on_scr.x() + 1.0) * 0.5 * w);
        pos_on_scr.set_y((1.0 - pos_on_scr.y()) * 0.5 * h);
        let equation_val = (x - pos_on_scr.x()).powi(2) + (y - pos_on_scr.y()).powi(2);
        let radius = BODY_GFX_SCALE * o.get_radius() as f32;
        if equation_val < ( radius / state.view_scale * (w + h) / 2.0).powi(2) {
            state.selected = o.get_id() as i64
        }
    }
}
