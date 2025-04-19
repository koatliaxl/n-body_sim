use crate::{Command, Msg, State};
use mat_vec::Vector3;
//use n_body_sim::ObjectType::*;
use n_body_sim::split_task_length;
use n_body_sim::{Object, ID_TABLE};
//use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};

pub use parallel::*;

mod parallel;

//static mut OBJECT_ID_TABLE: ObjectIdTable = ObjectIdTable::new();

pub struct World {
    pub objects: Vec<Object>,
    pub forces: Vec<Vector3<f64>>,
    pub obj_mirror: Vec<Arc<Mutex<ObjBuffer>>>,
}

pub struct ObjBuffer {
    pub objects: Vec<Object>,
    pub forces: Vec<Vector3<f64>>,
    pub task: usize,
    pub begin: usize,
}

pub fn next_step(world: &mut World, delta_t: f64, state: &State) {
    let tasks = split_task_length(world.objects.len(), state.workers.len());
    let mut offset = 0;
    for i in 0..tasks.len() {
        if let Ok(mut guard) = world.obj_mirror[i].lock() {
            let mirror = &mut guard.objects;
            mirror.clear();
            for obj in &world.objects {
                mirror.push(obj.clone())
            }
            guard.task = tasks[i];
            guard.begin = offset;
            offset += tasks[i]
        }
    }
    for sender in &state.to_workers {
        sender
            .send(Msg::NewTask { delta_t })
            .expect("main: failed to send msg.");
    }
}

pub fn check_if_tasks_finished(state: &mut State) {
    if let Ok(msg) = state.from_workers.try_recv() {
        if let Msg::TaskFinished = msg {
            state.received += 1;
        } else {
            panic!("received wrong message")
        }
    }
}

pub fn update_world(world: &mut World) {
    let mut i = 0;
    for mir in &world.obj_mirror {
        let mut guard = mir.lock().expect("main: lock not acquired");
        let ObjBuffer {
            ref mut objects,
            task,
            ..
        } = *guard;
        for j in 0..task {
            world.objects[i + j] = objects[i + j].clone();
        }
        i += task
    }
}

pub fn apply_commands(world: &mut World, state: &mut State) {
    while state.command_queue.len() > 0 {
        if let Some(command) = state.command_queue.pop_back() {
            match command {
                Command::Create {
                    pos,
                    vel,
                    mass,
                    class,
                } => {
                    let body = Object::new_by_vec3(pos, vel, mass, class);
                    world.objects.push(body);
                }
                Command::Delete { id } => {
                    let mut to_delete = 0;
                    for i in 0..world.objects.len() {
                        let body = &world.objects[i];
                        if body.get_id() == id {
                            //world.objects.swap_remove(i);
                            to_delete = i;
                            unsafe { ID_TABLE.release_id(id) }
                            break;
                        }
                    }
                    world.objects.swap_remove(to_delete);
                }
            }
        }
    }
}
