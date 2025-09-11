use crate::sim::World;
use crate::Msg;
use core::time::Duration;
use mat_vec::Vector3;
use n_body_sim::{Body, BodyType};
use std::collections::VecDeque;
use std::sync::mpsc::{Receiver, Sender};
use std::thread::JoinHandle;
use std::time::Instant;

//static NUM_THREADS: u64 =

pub mod prediction;
pub mod state;

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
    pub task_done_count: usize,
    pub update_processed: bool,
    pub workers: Vec<JoinHandle<()>>,
    pub selected: i64,
    pub new_obj_mass: f64,
    pub command_queue: VecDeque<Command>,
    //pub redraw_requested: bool,
    pub update_ui_requested: bool,
    pub prediction: Prediction,
}

pub struct Prediction {
    pub trajectory: VecDeque<Vector3<f64>>,
    pub state: World,
    pub history: VecDeque<Vec<Body>>,
    pub selected_ceased_to_exist_on: isize,
    pub task_done_count: usize,
    pub workers: Vec<JoinHandle<()>>,
    pub to_workers: Vec<Sender<Msg>>,
    pub from_workers: Receiver<Msg>,
    //pub devalidated: bool,
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
    pub prediction: bool,
}

pub struct Config {
    pub prediction_steps: usize,
}

pub struct Statistic {
    pub last_update_took: Duration,
    pub upd_measure_history: VecDeque<Duration>,
    pub upd_measure_num: usize,
    pub on_prediction_mode: bool,
}

/*pub enum UpdateMode {
    Normal,
    GettingFromPrediction,
MakingPrediction,
}*/

impl Config {
    pub fn new() -> Config {
        Config {
            prediction_steps: 100,
        }
    }
}

impl Statistic {
    pub fn new() -> Statistic {
        Statistic {
            last_update_took: Duration::from_secs(0),
            upd_measure_history: VecDeque::new(),
            upd_measure_num: 180,
            on_prediction_mode: false,
        }
    }

    pub fn add_upd_measure(&mut self, measure: Duration) {
        self.upd_measure_history.push_back(measure);
        if self.upd_measure_history.len() > self.upd_measure_num {
            let over = self.upd_measure_history.len() - self.upd_measure_num;
            self.upd_measure_history.drain(0..over - 1);
        }
    }

    pub fn reset_upd_measures(&mut self) {
        self.upd_measure_history.clear();
    }

    pub fn on_prediction_mode(&mut self, is: bool) {
        self.switch_mode(is)
    }

    fn switch_mode(&mut self, pred: bool) {
        if pred {
            if !self.on_prediction_mode {
                self.on_prediction_mode = true;
                self.reset_upd_measures();
            }
        } else {
            if self.on_prediction_mode {
                self.on_prediction_mode = false;
                self.reset_upd_measures();
            }
        }
    }
}
