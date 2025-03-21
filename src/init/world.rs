use crate::ObjectType::Massive;
use crate::{Object, World};
use mat_vec::Vector3;

pub fn init_world() -> World {
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
    ];
    World { objects }
}
