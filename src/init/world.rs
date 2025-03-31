use crate::{ObjBuffer, Object, World};
use mat_vec::Vector3;
use n_body_sim::ObjectType::Massive;
use std::sync::{Arc, Mutex};

pub fn init_world(number_threads: usize) -> World {
    let objects = vec![
        Object {
            pos: Vector3::new(3.5, 3.5, 0.0),
            vel: Vector3::new(1.8, -1.8, 0.0),
            mass: 1.0,
            class: Massive,
        },
        Object {
            pos: Vector3::new(0.0, 0.0, 0.0),
            vel: Vector3::new(0.0, 0.0, 0.0),
            mass: 50.0,
            class: Massive,
        },
        Object {
            pos: Vector3::new(-8.0, -8.0, 0.0),
            vel: Vector3::new(-1.0, 1.0, 0.0),
            mass: 2.0,
            class: Massive,
        },
    ];
    let forces = vec![Vector3::new(0.0, 0.0, 0.0); objects.len()];
    /*let obj_bufs = vec![
        Arc::new(Mutex::new(ObjBuffer {
            objects: Vec::new(),
            forces: Vec::new(),
        }));
        number_threads
    ];*/
    let mut obj_bufs = Vec::new();
    for _ in 0..number_threads {
        obj_bufs.push(Arc::new(Mutex::new(ObjBuffer {
            objects: Vec::new(),
            forces: Vec::new(),
            task: 0,
            begin: 0,
        })))
    }
    World {
        objects,
        forces,
        obj_mirror: obj_bufs,
    }
}
