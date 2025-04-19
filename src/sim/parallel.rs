use crate::{Msg, ObjBuffer, ThreadConfig};
use mat_vec::Vector3;
use n_body_sim::Object;
use n_body_sim::ObjectType::*;
//use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};

pub fn compute_in_parallel(
    /*receiver: Receiver<Msg>,
    sender: Sender<Msg>,*/
    th_cfg: ThreadConfig,
    mirror: Arc<Mutex<ObjBuffer>>,
) {
    loop {
        let msg = th_cfg
            .receiver
            .recv()
            .expect("worker: channel disconnected");
        if let Msg::NewTask { delta_t } = msg {
            let lock = mirror.lock();
            let mut guard = lock.expect("worker: lock not acquired");
            let ObjBuffer {
                ref mut objects,
                ref mut forces,
                task,
                begin,
            } = *guard;
            compute_forces(objects, forces, task, begin);
            apply_forces(objects, forces, delta_t, task, begin);
            th_cfg
                .sender
                .send(Msg::TaskFinished)
                .expect("worker: failed to send msg.");
        } else if let Msg::Exit = msg {
            break;
        } else {
            panic!("received wrong message")
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
