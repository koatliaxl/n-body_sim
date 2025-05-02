use crate::{Msg, ObjBuffer, ThreadConfig, /*BODY_RADIUS, BODY_RADIUS_SQR*/};
use mat_vec::Vector3;
use n_body_sim::BodyType::*;
use n_body_sim::{Body, Collision};
use std::collections::HashMap;
//use std::sync::mpsc::{Receiver, Sender};
//use n_body_sim::Collision::*;
use n_body_sim::SuspectCollChange::*;
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
                ref mut collisions,
            } = *guard;
            let bodies = bodies
                .lock()
                .expect("Worker: lock not acquired for parallel read");

            collisions.clear();
            prepare_changes(&bodies, changes, task, begin);
            compute_forces(&bodies, changes, forces, /*task, begin,*/ collisions);
            check_suspicion_hitboxes(&bodies, changes, delta_t);
            move_bodies(changes, forces, delta_t /*, task, begin*/);
            check_for_collisions(&bodies, changes, collisions);

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

fn check_for_collisions(
    bodies: &Vec<Body>,
    changes: &mut Vec<Body>,
    collisions: &mut HashMap<u64, Collision>,
) {
    //collisions.clear();
    for body in changes {
        if let Some(body_2) = body.check_for_collision(bodies) {
            let diff = body.pos - body_2.pos;
            let dist = diff.length();
            if dist < body.get_radius() {
                add_to_collisions(collisions, body, body_2)
            }
        }
    }
}

fn move_bodies(changes: &mut Vec<Body>, forces: &Vec<Vector3<f64>>, delta_t: f64) {
    for i in 0..changes.len() {
        let body = &mut changes[i];
        let force = &forces[i];
        match body.class {
            Massive | Light => {
                body.vel += *force * (1.0 / body.mass) * delta_t;
                body.pos += body.vel * delta_t;
            }
            _ => (),
        }
    }
}

fn check_suspicion_hitboxes(bodies: &Vec<Body>, changes: &mut Vec<Body>, delta_t: f64) {
    for body in changes {
        for body_2 in bodies {
            // ATTENTION! The next thing bellow might appear very murky at the first glance, but
            // wait, before thinking or doing something, it will be explained bellow.
            //   This is to prevent of the duplicate collision (with reverse body IDs) for being
            // added and tracked. For this, a some mechanism is needed that for any pair
            // of bodies IDs choose one order but not another. Compare is used, because "not equal"
            // check is already being used in this place, to prevent the collision check of
            // the body with itself.
            if body.get_id() > body_2.get_id() {
                let coord_diff = body_2.pos - body.pos;
                let dot_prod = coord_diff % body.vel; // dot product
                if dot_prod > 0.0 {
                    // = body in some degree shortens distance
                    let diff = coord_diff - body.vel;
                    // if velocity is comparable to distance:
                    if diff.x() < body.vel.x() || diff.y() < body.vel.y() {
                        body.suspect_collision(delta_t, body_2.get_id(), Increase);
                    } /*else {
                      }*/
                } else {
                    body.suspect_collision(delta_t, body_2.get_id(), Decrease);
                }
            }
        }
    }
}

fn prepare_changes(bodies: &Vec<Body>, changes: &mut Vec<Body>, task: usize, begin: usize) {
    changes.clear();
    for i in begin..task + begin {
        changes.push(bodies[i].clone())
    }
}

fn compute_forces(
    bodies: &Vec<Body>,
    changes: &mut Vec<Body>,
    forces: &mut Vec<Vector3<f64>>,
    /*task: usize,
    begin: usize,*/
    collisions: &mut HashMap<u64, Collision>,
) {
    forces.clear();
    for body in changes {
        //let body = &bodies[i];
        if body.class == Massive || body.class == Light {
            let mut total_force = Vector3::default();
            'l1: for j in 0..bodies.len() {
                let body_2 = &bodies[j];
                if body.get_id() != body_2.get_id() && body_2.class == Massive {
                    let displacement = body.pos - body_2.pos;
                    let dist_sqr = displacement.x().powi(2) + displacement.y().powi(2);
                    let rad_sqr = body.get_radius().powi(2);
                    if dist_sqr < rad_sqr && body.get_id() > body_2.get_id() {
                        add_to_collisions(collisions, body, body_2);
                        break 'l1;
                    }
                    //let dist = dist_sqr.sqrt();
                    let dir = -displacement.normalize();
                    total_force += dir * body.mass * body_2.mass * (1.0 / dist_sqr);
                }
            }
            forces.push(total_force)
        }
    }
}

fn add_to_collisions(collisions: &mut HashMap<u64, Collision>, body: &mut Body, to_body: &Body) {
    if let Some(Collision {
        mass,
        vel, /* rust fmt force vertical */
    }) = collisions.get_mut(&to_body.get_id())
    {
        let rel_vel = to_body.vel - body.vel;
        let momentum_1 = *mass * *vel;
        let momentum_2 = rel_vel * body.mass;
        *vel = (momentum_1 + momentum_2) * (1.0 / (*mass + body.mass));
        *mass += body.mass;
        //println!("total mass: {}", *mass)
    } else {
        collisions.insert(
            to_body.get_id(),
            Collision {
                mass: body.mass,
                vel: to_body.vel - body.vel,
            },
        );
        //println!("total mass: {}", body.mass)
    }
    body.class = Removed;
    //println!("collision happened")
}
