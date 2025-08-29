use crate::state_and_cfg::RunState::*;
use crate::state_and_cfg::*;
use crate::{/*update_gui,*/ Msg, World};
use glfw::Action::{Press, Release /*Repeat*/};
use glfw::MouseButton::Button1 as LeftButton;
use glfw::MouseButton::Button2 as RightButton;
use glfw::WindowEvent::{CursorPos, MouseButton, Scroll, Size};
use glfw::{Key, Window, WindowEvent};
use n_body_sim::{gl, GlData};
use std::sync::mpsc::Receiver;

pub use create::*;
use n_body_sim::gui::RootGIE;
pub use select::*;
pub use view::*;

pub mod create;
pub mod select;
pub mod view;

pub fn handle_events(
    window: &mut Window,
    events: &Receiver<(f64, WindowEvent)>,
    state: &mut State,
    gl_data: &GlData,
    world: &World,
    _gui: &mut RootGIE,
) {
    if window.get_key(Key::Escape) == Press {
        window.set_should_close(true);
        for snd in &state.to_workers {
            snd.send(Msg::Exit).expect("main: failed to send msg.");
        }
        for snd in &state.prediction.to_workers {
            snd.send(Msg::Exit).expect("main: failed to send msg.");
        }
    }

    for (_, event) in glfw::flush_messages(&events) {
        match event {
            Size(w, h) => {
                unsafe { gl::Viewport(0, 0, w, h) }
                view_pos_changed(gl_data, state, (w, h) /*, world*//*, gui*/);
                view_scale_changed(gl_data, state, (w, h));
                window_size_changed(gl_data, (w, h));
            }
            MouseButton(button, action, _) => {
                if button == LeftButton && action == Press {
                    state.left_mouse_bt_was_pressed = true;
                    select_obj(state, &world, window.get_size() /*, gui*/);
                    state.cursor_pos_when_press = state.last_cursor_pos;
                }
                if button == LeftButton && action == Release {
                    state.left_mouse_bt_was_pressed = false;
                    state.view_pos = state.new_view_pos;
                    if window.get_key(Key::C) == Press {
                        create_body(state, window.get_size())
                    }
                }
                if button == RightButton && action == Press {
                    state.selected = -1;
                    state.prediction.history.clear();
                    state.prediction.selected_ceased_to_exist_on = -1;
                }
            }
            CursorPos(x, y) => {
                state.last_cursor_pos = (x, y);
                if state.left_mouse_bt_was_pressed {
                    view_pos_changed(&gl_data, state, window.get_size() /*world,*/ /*gui*/)
                }
            }
            Scroll(_, s) => {
                state.view_scale -= s as f32 * 0.8;
                //println!("view scale: {}", state.view_scale);
                view_scale_changed(&gl_data, &state, window.get_size());
                state.update_ui_requested = true;
            }
            WindowEvent::Key(key, _, action, _modifiers) => match key {
                Key::P | Key::Pause if action == Release => match state.run_state {
                    Run => state.run_state = Pause,
                    Pause => state.run_state = Run,
                    Stop => state.run_state = Pause,
                },
                Key::C if action == Press => println!("new obj. mass: {}", state.new_obj_mass),
                Key::Minus if action == Press => {
                    if window.get_key(Key::C) == Press {
                        state.new_obj_mass *= 0.8;
                        println!("new obj. mass: {}", state.new_obj_mass)
                    }
                }
                Key::Equal if action == Press => {
                    if window.get_key(Key::C) == Press {
                        state.new_obj_mass *= 1.25;
                        println!("new obj. mass: {}", state.new_obj_mass)
                    }
                }
                Key::Delete if action == Release => {
                    if state.selected >= 0 {
                        state.command_queue.push_front(Command::Delete {
                            id: state.selected as u64,
                        })
                    }
                }
                _ => (),
            },
            _ => (),
        }
    }
}
