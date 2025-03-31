use crate::{Msg, State};
use mat_vec::Vector3;
use n_body_sim::split_task_length;
use n_body_sim::Object;
use n_body_sim::ObjectType::*;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};

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
        sender.send(Msg::NewTask { delta_t });
    }
}

pub fn check_if_tasks_finished(state: &mut State) {
    if let Ok(msg) = state.from_workers.try_recv() {
        if let Msg::TaskFinished = msg {
            state.received += 1;
        } else {
            unreachable!()
        }
    }
}

pub fn update_world(world: &mut World) {
    let mut i = 0;
    for mir in &world.obj_mirror {
        if let Ok(mut guard) = mir.lock() {
            let ObjBuffer {
                ref mut objects,
                task,
                ..
            } = *guard;
            for j in 0..task {
                world.objects[i + j] = objects[i + j].clone();
                i += 1
            }
        } else {
            eprintln!("main: lock not acquired")
        }
    }
}

pub fn compute_in_parallel(
    receiver: Receiver<Msg>,
    sender: Sender<Msg>,
    mirror: Arc<Mutex<ObjBuffer>>,
) {
    loop {
        if let Ok(msg) = receiver.recv() {
            if let Msg::NewTask { delta_t } = msg {
                let lock = mirror.lock();
                if let Ok(mut guard) = lock {
                    let ObjBuffer {
                        ref mut objects,
                        ref mut forces,
                        task,
                        begin,
                    } = *guard;
                    compute_forces(objects, forces, task, begin);
                    apply_forces(objects, forces, delta_t, task, begin);
                    sender.send(Msg::TaskFinished);
                } else {
                    eprintln!("worker: lock not acquired")
                }
            } else if let Msg::Exit = msg {
                break;
            } else {
                unreachable!()
            }
        } else {
            eprintln!("channel disconnected")
        }
    }
}

pub fn apply_forces(
    objects: &mut Vec<Object>,
    forces: &Vec<Vector3<f64>>,
    delta_t: f64,
    task: usize,
    begin: usize,
) {
    for i in begin..task + begin {
        let obj = &mut objects[i];
        let force = &forces[i - begin];
        match obj.class {
            Massive | Light => {
                obj.vel += *force * (1.0 / obj.mass) * delta_t;
                obj.pos += obj.vel * delta_t;
            }
            _ => (),
        }
    }
}

pub fn compute_forces(
    objects: &Vec<Object>,
    forces: &mut Vec<Vector3<f64>>,
    task: usize,
    begin: usize,
) {
    forces.clear();
    for i in begin..task + begin {
        let obj = &objects[i];
        forces.push(Vector3::default());
        if obj.class == Massive || obj.class == Light {
            let mut total_force = Vector3::default();
            for j in 0..objects.len() {
                let obj_2 = &objects[j];
                if j != i {
                    if let Massive = &obj_2.class {
                        let displacement = obj.pos - obj_2.pos;
                        let dist_sqr = displacement.x().powi(2) + displacement.y().powi(2);
                        //let dist = dist_sqr.sqrt();
                        let dir = -displacement.normalize();
                        total_force += dir * obj.mass * obj_2.mass * (1.0 / dist_sqr);
                    }
                }
            }
            forces[i - begin] = total_force;
        }
    }
}
