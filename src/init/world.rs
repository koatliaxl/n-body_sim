use crate::{Body, ObjBuffer, World};
//use mat_vec::Vector3;
//use n_body_sim::ObjectType::Massive;
use std::sync::{Arc, Mutex};

pub fn init_world(number_threads: usize) -> World {
    let objects = vec![
        Body::new(3.5, 3.5, 1.8, -1.8, 1.0),
        Body::new(0.0, 0.0, 0.0, 0.0, 50.0),
        Body::new(-8.0, -8.0, -1.0, 1.0, 2.0),
    ];
    let objects = Arc::new(Mutex::new(objects));
    //let forces = vec![Vector3::default(); objects.len()];
    let forces = Vec::new();
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
            par_read: Arc::clone(&objects),
            changes: Vec::new(),
            forces: Vec::new(),
            task: 0,
            begin: 0,
        })))
    }
    World {
        bodies: objects,
        forces,
        obj_mirror: obj_bufs,
    }
}
