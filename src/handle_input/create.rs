use crate::{Command, State};
use mat_vec::{Matrix4x4, Vector3, Vector4};
use n_body_sim::BodyType::Massive;

pub fn create_body(state: &mut State, window_size: (i32, i32)) {
    let (x, y) = (
        state.last_cursor_pos.0 as f32,
        state.last_cursor_pos.1 as f32,
    );
    let (w, h) = (window_size.0 as f32, window_size.1 as f32);
    // screen coordinates in range [-1; -1]
    let screen_x = 2.0 * x / w - 1.0;
    let screen_y = 1.0 - 2.0 * y / h;
    let screen_pos = Vector4::new(screen_x as f32, screen_y as f32, 0.0, 1.0);
    let inv_view = Matrix4x4::<f32>::inv_LookAt_matrix(
        state.view_pos,
        Vector3::new(0.0, 0.0, -1.0),
        Vector3::new(0.0, 1.0, 0.0),
    );
    let ratio = w / h;
    let inv_proj = Matrix4x4::<f32>::inv_orthographic_projection(
        state.view_scale * ratio,
        state.view_scale,
        10.0,
        0.1,
    );
    let world_pos = inv_view * inv_proj * screen_pos; //todo vec3 conv.
    let (wx, wy, _, _) = world_pos.get_components();
    let command = Command::Create {
        pos: Vector3::new(wx as f64, wy as f64, 0.0),
        vel: Vector3::new(0.0, 0.0, 0.0),
        mass: state.new_obj_mass,
        class: Massive,
    };
    state.command_queue.push_front(command)
}
