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
    TaskFinished {},
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
    let (mut world, prediction) = init_world(num_of_threads);
    init_draw(&mut gl_data);
    init_glyphs(&mut gl_data);
    let mut state = State::new(&world.obj_mirror, prediction);
    let mut gui = init_gui();
    let cfg = Config::new();

    view_pos_changed(&gl_data, &mut state, window.get_size());
    view_scale_changed(&gl_data, &state, window.get_size());
    window_size_changed(&gl_data, window.get_size());

    let mut tic_duration = 0.0;
    //let mut update_processed = true;
    let mut last_frame_time = Instant::now();
    let mut between_frames = 0.0;
    while !window.should_close() {
        if state.fps_changed {
            between_frames = 1000.0 / state.fps as f64;
            state.fps_changed = false;
        }
        let since_last_frame = last_frame_time.elapsed();
        if since_last_frame.as_secs_f64() * 1000.0 >= between_frames && state.run_state != Stop
        /*|| state.redraw_requested*/
        {
            if state.selected >= 0 {
                predict(&world, &mut state, &cfg, tic_duration / 1000.0);
            }
            draw(&gl_data, &world, &state, window.get_size(), &cfg);
            gui.draw(&gl_data);
            window.swap_buffers();
            last_frame_time = Instant::now();
            //state.redraw_requested = false
        }
        if state.ups_changed {
            tic_duration = 1000.0 / state.ups as f64;
            state.ups_changed = false;
        }
        let since_last_upd = state.last_upd_time.elapsed();
        if since_last_upd.as_secs_f64() * 1000.0 >= tic_duration
            && state.run_state == Run
            && state.update_processed
        {
            if state.prediction.history.is_empty() {
                println!("task given (non pred.)");
                begin_next_step(&mut world, tic_duration / 1000.0, &mut state, false);
                state.update_processed = false
            } else {
                let next_step = if let Some(bodies) = state.progress_to_next_step() {
                    bodies
                } else {
                    unreachable!(
                        "because element must be always popped when deque is not empty, \
                    and condition that history is not empty is prerequisite to that code"
                    )
                };
                *world.bodies.lock().expect(
                    "applying next step from prediction: lock on bodies must be acquired",
                ) = next_step;
                //state.prediction.trajectory.pop_front();
                state.update_ui_requested = true;
            }
            state.last_upd_time = Instant::now();
        }
        if state.task_done_count < state.workers.len() {
            check_if_tasks_finished(&mut state, false);
        } else {
            update_world(&mut world);
            apply_collisions(&mut world);
            apply_commands(&mut world, &mut state);
            state.update_processed = true;
            state.task_done_count = 0;
            state.update_ui_requested = true;
            state.progress_to_next_step();
        }
        glfw.poll_events();
        handle_events(&mut window, &events, &mut state, &gl_data, &world, &mut gui);
        if state.update_ui_requested {
            update_gui(&mut state, &world, window.get_size(), &mut gui);
            state.update_ui_requested = false
        }
    }
    for jh in state.workers {
        jh.join().expect("failed to join worker");
    }
    for jh in state.prediction.workers {
        jh.join().expect("failed to join worker");
    }
}
