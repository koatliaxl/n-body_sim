use crate::world::init_world;
use draw::draw;
use glfw::Context;
use handle_input::*;
use init::*;
use n_body_sim::GlData;
use n_body_sim::{gl, Body};
use sim::*;
use state_and_cfg::RunState::*;
use state_and_cfg::*;
use std::time::Instant;

mod draw;
mod handle_input;
mod init;
mod sim;
mod state_and_cfg;
pub mod update;

pub use update::*;

pub enum Msg {
    NewTask { delta_t: f64 },
    TaskFinished,
    Exit,
}

unsafe impl Send for Msg {}

fn main() {
    let mut gl_data = GlData::new();
    let (mut glfw, mut window, events) = init_glfw();
    gl::load_with(|s| window.get_proc_address(s));
    init_open_gl(window.get_size());
    init_shaders(&mut gl_data);
    let num_of_threads = 3;
    let mut world = init_world(num_of_threads);
    init_draw(&mut gl_data);
    init_glyphs(&mut gl_data);
    let mut state = State::new(&world.obj_mirror);
    let mut gui = init_gui();

    view_pos_changed(&gl_data, &mut state, window.get_size(), &world, &mut gui);
    view_scale_changed(&gl_data, &state, window.get_size());

    let mut tic_duration = 0.0;
    let mut update_processed = true;
    let mut last_frame_time = Instant::now();
    let mut between_frames = 0.0;
    while !window.should_close() {
        if state.fps_changed {
            between_frames = 1000.0 / state.fps as f64;
            state.fps_changed = false;
        }
        let since_last_frame = last_frame_time.elapsed();
        if since_last_frame.as_secs_f64() * 1000.0 >= between_frames || state.run_state != Stop {
            update_gui(&mut state, &world, window.get_size(), &mut gui);
            draw(&gl_data, &world, &state, window.get_size());
            gui.draw(&gl_data);
            window.swap_buffers();
            last_frame_time = Instant::now();
        }
        if state.ups_changed {
            tic_duration = 1000.0 / state.ups as f64;
            state.ups_changed = false;
        }
        let since_last_upd = state.last_upd_time.elapsed();
        if since_last_upd.as_secs_f64() * 1000.0 >= tic_duration
            && state.run_state == Run
            && update_processed
        {
            begin_next_step(&mut world, tic_duration / 1000.0, &mut state);
            state.last_upd_time = Instant::now();
            update_processed = false
        }
        if state.received < state.workers.len() {
            check_if_tasks_finished(&mut state);
        } else {
            update_world(&mut world);
            apply_collisions(&mut world);
            apply_commands(&mut world, &mut state);
            update_processed = true;
            state.received = 0
        }
        glfw.poll_events();
        handle_events(&mut window, &events, &mut state, &gl_data, &world, &mut gui);
    }
    for jh in state.workers {
        jh.join().expect("failed to join worker");
    }
}
