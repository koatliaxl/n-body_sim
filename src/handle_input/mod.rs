use crate::gl_data::GlData;
use crate::state_and_cfg::RunState::*;
use crate::state_and_cfg::*;
use crate::Msg;
use glfw::Action::Release;
use glfw::MouseButton::Button1 as LeftButton;
use glfw::WindowEvent::{CursorPos, MouseButton /* as MousePress*/, Scroll, Size};
use glfw::{Action, Key, Window, WindowEvent};
use n_body_sim::gl;
use std::sync::mpsc::Receiver;

pub use view::*;

pub mod view;

pub fn handle_events(
    window: &mut Window,
    events: &Receiver<(f64, WindowEvent)>,
    state: &mut State,
    gl_data: &GlData,
) {
    if window.get_key(Key::Escape) == Action::Press {
        window.set_should_close(true);
        for snd in &state.to_workers {
            snd.send(Msg::Exit);
        }
    }
    for (_, event) in glfw::flush_messages(&events) {
        match event {
            Size(w, h) => unsafe { gl::Viewport(0, 0, w, h) },
            MouseButton(button, action, _) => {
                if button == LeftButton && action == Action::Press {
                    state.left_mouse_bt_was_pressed = true;
                    state.cursor_pos_when_press = state.last_cursor_pos;
                }
                if button == LeftButton && action == Action::Release {
                    state.left_mouse_bt_was_pressed = false;
                    state.view_pos = state.new_view_pos;
                }
            }
            CursorPos(x, y) => {
                state.last_cursor_pos = (x, y);
                if state.left_mouse_bt_was_pressed {
                    view_pos_changed(&gl_data, state, window.get_size())
                }
            }
            Scroll(_, s) => {
                state.view_scale -= s as f32 * 0.9;
                view_scale_changed(&gl_data, &state, window.get_size())
            }
            WindowEvent::Key(key, _, action, _modifiers) => match key {
                Key::P | Key::Pause if action == Action::Release => {
                    println!("key 'P' pressed");
                    match state.run_state {
                        Run => state.run_state = Pause,
                        Pause => state.run_state = Run,
                        Stop => state.run_state = Pause,
                    }
                }
                _ => (),
            },
            _ => (),
        }
    }
}
