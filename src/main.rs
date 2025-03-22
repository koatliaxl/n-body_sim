extern crate core;

use init::*;
use std::sync::mpsc::Receiver;
//use std::sync::mpsc::{Receiver, Sender};
//use std::sync::{mpsc, Arc, Mutex};
//use std::thread;
use crate::gl_data::GlData;
use crate::world::init_world;
use draw::draw;
use glfw::MouseButton::Button1 as LeftButton;
use glfw::WindowEvent::{CursorPos, MouseButton /* as MousePress*/, Scroll, Size};
use glfw::{Action, Context, Key, Window, WindowEvent};
use handle_input::*;
use n_body_sim::gl;
use sim::*;
use state_and_cfg::RunState::{Run, Stop};
use state_and_cfg::*;
use std::time::Instant;
use glfw::Action::Release;
use crate::RunState::Pause;

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

    //let mut tic_processed = false;
    let mut tic_duration = 0.0;
    //let mut update_processed = false;
    //let mut new_frame = true;
    let mut last_frame_time = Instant::now();
    let mut between_frames = 0.0;
    while !window.should_close() {
        if state.fps_changed {
            between_frames = 1000.0 / state.fps as f64;
        }
        let since_last_frame = last_frame_time.elapsed();
        if since_last_frame.as_secs_f64() * 1000.0 >= between_frames || state.run_state != Stop {
            unsafe {
                draw(&gl_data, &world, &state);
            }
            window.swap_buffers();
            last_frame_time = Instant::now();
            //new_frame = false;
        }
        if state.ups_changed {
            tic_duration = 1000.0 / state.ups as f64;
            state.ups_changed = false;
        }
        let since_last_upd = state.last_upd_time.elapsed();
        if since_last_upd.as_secs_f64() * 1000.0 >= tic_duration && state.run_state == Run {
            process_step(&mut world, tic_duration / 1000.0);
            state.last_upd_time = Instant::now();
            //tic_processed = false;
        }
        glfw.poll_events();
        handle_events(&mut window, &events, &mut state, &gl_data);
        //apply commands()
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
                    match state.run_state  {
                        Run => state.run_state = Pause,
                        Pause => state.run_state = Run,
                        Stop => state.run_state = Pause
                    }
                }
                _ => (),
            }
            _ => (),
        }
    }
}
