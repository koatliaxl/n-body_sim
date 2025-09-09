use crate::{Body, ObjBuffer, World};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub fn init_world(number_threads: usize) -> (World, World) {
    let objects = vec![
        Body::new(3.5, 3.5, 1.8, -1.8, 1.0),
        Body::new(0.0, 0.0, 0.0, 0.0, 50.0),
        Body::new(-8.0, -8.0, -1.0, 1.0, 2.0),
    ];
    let objects = Arc::new(Mutex::new(objects));
    let forces = Vec::new();
    // The next doesn't work as intended, in the vector initialized that way all Arcs are
    // "connected", i.e. are references to the same value!
    /*let obj_bufs = vec![
        Arc::new(Mutex::new(ObjBuffer {
            objects: Vec::new(),
            forces: Vec::new(),
        }));
        number_threads
    ];*/
    let prediction_state = Arc::new(Mutex::new(Vec::new()));
    let mut obj_bufs = Vec::new();
    let mut pred_bufs = Vec::new();
    for _ in 0..number_threads {
        obj_bufs.push(Arc::new(Mutex::new(ObjBuffer {
            par_read: Arc::clone(&objects),
            changes: Vec::new(),
            forces: Vec::new(),
            task: 0,
            begin: 0,
            collisions: HashMap::new(),
        })));
        pred_bufs.push(Arc::new(Mutex::new(ObjBuffer {
            par_read: Arc::clone(&prediction_state),
            changes: Vec::new(),
            forces: Vec::new(),
            task: 0,
            begin: 0,
            collisions: HashMap::new(),
        })))
    }
    (
        World {
            bodies: objects,
            forces,
            obj_mirror: obj_bufs,
        },
        World {
            bodies: prediction_state,
            forces: Vec::new(),
            obj_mirror: pred_bufs,
        },
    )
}
