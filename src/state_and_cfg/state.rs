use crate::{compute_in_parallel, ObjBuffer, Prediction, RunState, State, ThreadConfig, World};
use mat_vec::Vector3;
use std::collections::VecDeque;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Instant;

impl State {
    pub fn new(
        /*number_of_threads: u32*/ data_mirrors: &Vec<Arc<Mutex<ObjBuffer>>>,
        prediction_holder: World,
    ) -> State {
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
                prediction: false,
            };
            let jh = thread::spawn(move || {
                compute_in_parallel(th_cfg, mirror);
            });
            to_workers.push(to_worker);
            jh_vec.push(jh)
        }
        //state.from_workers = from_workers;

        let mut to_pred_workers = Vec::new();
        let mut jh_pred = Vec::new();
        let (to_main_from_pw, from_pred_w) = mpsc::channel();
        let last_id = data_mirrors.len();
        for i in last_id..last_id + prediction_holder.obj_mirror.len() {
            let (to_pred_w, from_main) = mpsc::channel();
            let mirror = Arc::clone(&prediction_holder.obj_mirror[i - last_id]);
            let th_cfg = ThreadConfig {
                receiver: from_main,
                sender: to_main_from_pw.clone(),
                id: i,
                prediction: true,
            };
            let jh = thread::spawn(move || compute_in_parallel(th_cfg, mirror));
            to_pred_workers.push(to_pred_w);
            jh_pred.push(jh);
        }

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
            task_done_count: 0,
            workers: jh_vec,
            selected: -1,
            new_obj_mass: 1.0,
            command_queue: VecDeque::new(),
            //redraw_requested: false,
            update_ui_requested: false,
            prediction: Prediction {
                trajectory: VecDeque::new(),
                state: prediction_holder,
                history: VecDeque::new(),
                selected_ceased_to_exist_on: -1,
                task_done_count: 0,
                workers: jh_pred,
                to_workers: to_pred_workers,
                from_workers: from_pred_w,
                //devalidated: false,
            },
            update_processed: true,
        }
    }
}
