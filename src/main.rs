extern crate core;

use init::*;
use std::sync::mpsc::Receiver;
//use std::sync::mpsc::{Receiver, Sender};
//use std::sync::{mpsc, Arc, Mutex};
//use std::thread;
use crate::gl_data::GlData;
use crate::world::init_world;
use draw::draw;
use glfw::WindowEvent::{CursorPos, Scroll, Size};
use glfw::{Action, Context, Key, MouseButton, Window, WindowEvent};
use handle_input::*;
use n_body_sim::gl;
use sim::*;
use state_and_cfg::*;
use std::time::Instant;

mod draw;
mod gl_data;
mod handle_input;
mod init;
mod sim;
mod state_and_cfg;

/*enum Msg {
    RequestRedraw,
    SwapBuffers,
    //Stop,
}*/

fn main() {
    let mut gl_data = GlData::new();
    let (mut glfw, mut window, events) = init_glfw();
    gl::load_with(|s| window.get_proc_address(s));
    init_open_gl(window.get_size());
    init_shaders(&mut gl_data);
    let mut world = init_world();
    unsafe {
        init_draw(&mut gl_data);
    }
    let mut state = State::new();

    view_pos_changed(&gl_data, &mut state, window.get_size());
    view_scale_changed(&gl_data, &state, window.get_size());

    let mut tic_processed = false;
    let mut tic_duration = 0.0;
    while !window.should_close() {
        let elapsed = state.tic_start_time.elapsed();
        if state.ups_changed {
            tic_duration = 1000.0 / state.ups as f64;
            state.ups_changed = false;
        }
        if elapsed.as_secs_f64() * 1000.0 >= tic_duration {
            state.tic_start_time = Instant::now();
            tic_processed = false;
        }
        if !tic_processed {
            //apply commands()
            process_step(&mut world, tic_duration / 1000.0);
            unsafe {
                draw(&gl_data, &world, &state);
            }
            window.swap_buffers();
            glfw.poll_events();
            tic_processed = true;
            handle_events(&mut window, &events, &mut state, &gl_data);
        }
    }
}

fn handle_events(
    window: &mut Window,
    events: &Receiver<(f64, WindowEvent)>,
    state: &mut State,
    gl_data: &GlData,
) {
    if window.get_key(Key::Escape) == Action::Press {
        window.set_should_close(true);
    }
    for (_, event) in glfw::flush_messages(&events) {
        match event {
            Size(w, h) => unsafe { gl::Viewport(0, 0, w, h) },
            WindowEvent::MouseButton(button, action, _) => {
                if button == MouseButton::Button1 && action == Action::Press {
                    state.left_mouse_bt_was_pressed = true;
                    state.cursor_pos_when_press = state.last_cursor_pos;
                }
                if button == MouseButton::Button1 && action == Action::Release {
                    /*if state.left_mouse_bt_was_pressed {
                        view_pos_changed(&gl_data, state, window.get_size())
                    }*/
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
            _ => (),
        }
    }
}
