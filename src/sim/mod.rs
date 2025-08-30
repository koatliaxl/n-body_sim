use crate::{Command, Msg, State};
use mat_vec::Vector3;
use n_body_sim::BodyType::*;
use n_body_sim::{split_task_length, Collision};
use n_body_sim::{Body, ID_TABLE};
use std::collections::HashMap;
//use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};

pub use parallel::*;
pub use predict::*;

mod parallel;
mod predict;

pub struct World {
    pub bodies: Arc<Mutex<Vec<Body>>>,
    pub forces: Vec<Vector3<f64>>,
    pub obj_mirror: Vec<Arc<Mutex<ObjBuffer>>>,
}

impl World {}

pub struct ObjBuffer {
    pub par_read: Arc<Mutex<Vec<Body>>>, // for the parallel reading, but consecutive write
    pub changes: Vec<Body>,
    pub forces: Vec<Vector3<f64>>,
    pub task: usize,
    pub begin: usize,
    pub collisions: HashMap<u64, Collision>,
    //pub prediction_state: Arc<Mutex<Vec<Body>>>,
}

pub fn begin_next_step(world: &World, delta_t: f64, state: &State, prediction_mode: bool) {
    let bodies = world.bodies.lock().expect("Main: failed to acquire lock");
    let tasks = split_task_length(bodies.len(), state.workers.len()); //todo potential bug
    let mut offset = 0;
    for i in 0..tasks.len() {
        let mut guard = world.obj_mirror[i]
            .lock()
            .expect(" Main: lock not acquired");
        guard.task = tasks[i];
        guard.begin = offset;
        offset += tasks[i];
    }
    let to_workers = if !prediction_mode {
        &state.to_workers
    } else {
        &state.prediction.to_workers
    };
    for sender in to_workers {
        sender
            .send(Msg::NewTask { delta_t })
            .expect("Main: failed to send msg.");
    }
}

pub fn check_if_tasks_finished(state: &mut State, prediction: bool) {
    if !prediction {
        if let Ok(msg) = state.from_workers.try_recv() {
            match msg {
                Msg::TaskFinished {} => state.task_done_count += 1,
                _ => panic!("Main: received wrong message"),
            }
        }
    } else {
        if let Ok(msg) = state.prediction.from_workers.try_recv() {
            match msg {
                Msg::TaskFinished {} => state.prediction.task_done_count += 1,
                _ => panic!("Main: received wrong message"),
            }
        }
    }
}

pub fn update_world(world: &World) {
    let mut bodies = world
        .bodies
        .lock()
        .expect("Main: lock not acquired on bodies");
    let mut i = 0;
    for mir in &world.obj_mirror {
        let mut guard = mir.lock().expect("Main: lock not acquired on obj. buffer");
        let ObjBuffer {
            ref mut changes,
            task,
            ..
        } = *guard;
        for j in 0..task {
            bodies[i + j] = changes[j].clone()
        }
        i += task
    }
}

pub fn apply_collisions(world: &World) {
    let mut bodies = world
        .bodies
        .lock()
        .expect("Main: lock not acquired on bodies");
    for mir in &world.obj_mirror {
        let mut guard = mir.lock().expect("Main: lock not acquired on obj. buffer");
        for (id, Collision { mass, vel }) in guard.collisions.iter_mut() {
            'inner: for body in bodies.iter_mut() {
                if *id == body.get_id() {
                    //let rel_vel = body.vel - *vel;
                    let momentum = *vel * *mass; // collision vel. is already relative
                    body.vel = momentum * (1.0 / (*mass + body.mass));
                    body.mass += *mass;
                    //println!("collision applied");
                    body.update_radius();
                    break 'inner;
                }
            }
        }
    }
    bodies.retain(|body| body.class != Removed)
}

pub fn apply_commands(world: &mut World, state: &mut State) {
    if state.command_queue.is_empty() {
        return;
    } else {
        state.prediction.devalidate_history()
    }
    let mut bodies = world
        .bodies
        .lock()
        .expect("Main: lock not acquired on bodies");
    while state.command_queue.len() > 0 {
        if let Some(command) = state.command_queue.pop_back() {
            match command {
                Command::Create {
                    pos,
                    vel,
                    mass,
                    class,
                } => {
                    let body = Body::new_by_vec3(pos, vel, mass, class);
                    bodies.push(body);
                }
                Command::Delete { id } => {
                    let mut to_delete = 0;
                    for i in 0..bodies.len() {
                        let body = &bodies[i];
                        if body.get_id() == id {
                            to_delete = i;
                            unsafe { ID_TABLE.release_id(id) }
                            break;
                        }
                    }
                    bodies.swap_remove(to_delete);
                }
            }
        }
    }
}
