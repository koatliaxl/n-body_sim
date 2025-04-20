use crate::{compute_in_parallel, Msg, ObjBuffer};
use mat_vec::Vector3;
use n_body_sim::BodyType;
use std::collections::VecDeque;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use std::time::Instant;

pub use gl_data::GlData;

mod gl_data;

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
    pub to_workers: Vec<Sender<Msg>>,
    pub from_workers: Receiver<Msg>,
    pub received: usize,
    pub workers: Vec<JoinHandle<()>>,
    pub selected: i64,
    pub new_obj_mass: f64,
    pub command_queue: VecDeque<Command>,
}

pub enum Command {
    Create {
        pos: Vector3<f64>,
        vel: Vector3<f64>,
        mass: f64,
        class: BodyType,
    },
    Delete {
        id: u64,
    },
}

pub struct ThreadConfig {
    pub receiver: Receiver<Msg>,
    pub sender: Sender<Msg>,
    pub id: usize,
}

impl State {
    pub fn new(/*number_of_threads: u32*/ data_mirrors: &Vec<Arc<Mutex<ObjBuffer>>>) -> State {
        let mut to_workers = Vec::new();
        let mut jh_vec = Vec::new();
        let (to_main, from_workers) = mpsc::channel();
        for i in 0..data_mirrors.len() {
            let (to_worker, rcv) = mpsc::channel();
            let mirror = Arc::clone(&data_mirrors[i]);
            let th_cfg = ThreadConfig {
                receiver: rcv,
                sender: to_main.clone(),
                id: i,
            };
            let jh = thread::spawn(move || {
                compute_in_parallel(th_cfg, mirror);
            });
            to_workers.push(to_worker);
            jh_vec.push(jh)
        }
        //state.from_workers = from_workers;
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
            to_workers,
            from_workers,
            received: 0,
            workers: jh_vec,
            selected: -1,
            new_obj_mass: 1.0,
            command_queue: VecDeque::new(),
        }
    }
}

/*struct Config {

}*/
