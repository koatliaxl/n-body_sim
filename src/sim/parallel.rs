use crate::{Msg, ObjBuffer, ThreadConfig};
use mat_vec::Vector3;
use n_body_sim::Body;
use n_body_sim::BodyType::*;
//use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};

pub fn compute_in_parallel(th_cfg: ThreadConfig, mirror: Arc<Mutex<ObjBuffer>>) {
    loop {
        let msg = th_cfg
            .receiver
            .recv()
            .expect("Worker: channel disconnected");
        if let Msg::NewTask { delta_t } = msg {
            let lock = mirror.lock();
            let mut guard = lock.expect("Worker: lock not acquired");
            let ObjBuffer {
                par_read: ref bodies,
                ref mut changes,
                ref mut forces,
                task,
                begin,
            } = *guard;
            let bodies = bodies
                .lock()
                .expect("Worker: lock not acquired for parallel read");
            compute_forces(&bodies, forces, task, begin);
            apply_forces(&bodies, changes, forces, delta_t, task, begin);
            th_cfg
                .sender
                .send(Msg::TaskFinished)
                .expect("Worker: failed to send msg.");
        } else if let Msg::Exit = msg {
            break;
        } else {
            panic!("Worker: Received wrong message")
        }
    }
}

pub fn apply_forces(
    bodies: &Vec<Body>,
    changes: &mut Vec<Body>,
    forces: &Vec<Vector3<f64>>,
    delta_t: f64,
    task: usize,
    begin: usize,
) {
    changes.clear();
    for i in begin..task + begin {
        let mut body = bodies[i].clone();
        let force = &forces[i - begin];
        match body.class {
            Massive | Light => {
                body.vel += *force * (1.0 / body.mass) * delta_t;
                body.pos += body.vel * delta_t;
                changes.push(body)
            }
            _ => (),
        }
    }
}

pub fn compute_forces(
    bodies: &Vec<Body>,
    forces: &mut Vec<Vector3<f64>>,
    task: usize,
    begin: usize,
) {
    forces.clear();
    for i in begin..task + begin {
        let body = &bodies[i];
        if body.class == Massive || body.class == Light {
            let mut total_force = Vector3::default();
            for j in 0..bodies.len() {
                let body_2 = &bodies[j];
                if j != i && body_2.class == Massive {
                    let displacement = body.pos - body_2.pos;
                    let dist_sqr = displacement.x().powi(2) + displacement.y().powi(2);
                    //let dist = dist_sqr.sqrt();
                    let dir = -displacement.normalize();
                    total_force += dir * body.mass * body_2.mass * (1.0 / dist_sqr);
                }
            }
            forces.push(total_force)
        }
    }
}
