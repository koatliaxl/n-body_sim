use crate::{compute_in_parallel, Msg, ObjBuffer};
use mat_vec::Vector3;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
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
    pub to_workers: Vec<Sender<Msg>>,
    pub from_workers: Receiver<Msg>,
    pub received: usize,
    pub workers: Vec<JoinHandle<()>>,
}

impl State {
    pub fn new(/*number_of_threads: u32*/ data_mirrors: &Vec<Arc<Mutex<ObjBuffer>>>) -> State {
        let mut to_workers = Vec::new();
        let mut jh_vec = Vec::new();
        let (to_main, from_workers) = mpsc::channel();
        for m in data_mirrors {
            let (to_worker, rcv) = mpsc::channel();
            let sender = to_main.clone();
            let mirror = Arc::clone(m);
            let jh = thread::spawn(move || {
                compute_in_parallel(rcv, sender, mirror);
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
        }
    }
}
