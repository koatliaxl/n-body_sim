use mat_vec::Vector3;
use std::time::Instant;

#[derive(PartialEq)]
pub enum RunState {
    Run,
    Pause,
    Stop,
}

pub struct State {
    pub start_time: Instant,
    pub last_upd_time: Instant,
    pub ups: u32,
    pub ups_changed: bool,
    pub fps: u32,
    pub fps_changed: bool,
    //pub new_frame: bool,
    pub view_pos: Vector3<f32>,
    pub new_view_pos: Vector3<f32>,
    pub left_mouse_bt_was_pressed: bool,
    pub cursor_pos_when_press: (f64, f64),
    pub last_cursor_pos: (f64, f64),
    pub view_scale: f32,
    pub run_state: RunState,
}

impl State {
    pub fn new() -> State {
        State {
            start_time: Instant::now(),
            last_upd_time: Instant::now(),
            ups: 25,
            ups_changed: true,
            fps: 60,
            fps_changed: true,
            view_pos: Vector3::new(0.0, 0.0, 0.9),
            left_mouse_bt_was_pressed: false,
            cursor_pos_when_press: (0.0, 0.0),
            last_cursor_pos: (0.0, 0.0),
            view_scale: 10.0,
            new_view_pos: Vector3::new(0.0, 0.0, 0.9),
            run_state: RunState::Run,
        }
    }
}
