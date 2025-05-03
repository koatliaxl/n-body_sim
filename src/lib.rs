use self::SuspectCollChange::*;
use mat_vec::Vector3;
use std::collections::HashMap;

pub use support::id_table::ObjectIdTable;

pub const SIZE_OF_GL_FLOAT: isize = std::mem::size_of::<gl::types::GLfloat>() as isize;
pub static SUSPECT_COLLISION_THRESHOLD: f64 = 0.1;
pub static BODY_DENSITY_VALUE: f64 = 0.4;

pub static mut ID_TABLE: ObjectIdTable = ObjectIdTable::new();

pub mod gl {
    include!("../gl/bindings.rs");
}

#[cfg(test)]
mod tests;
pub mod support {
    pub mod id_table;
}

pub struct Body {
    pub pos: Vector3<f64>,
    pub vel: Vector3<f64>,
    pub mass: f64,
    phys_radius: f64,
    pub class: BodyType,
    id: u64,
    suspect_for_collision: HashMap<u64, SuspectedCollision>,
}

#[derive(PartialEq, Copy, Clone)]
pub enum BodyType {
    Removed,
    Massive,
    Light,
}

#[derive(Copy, Clone)]
pub struct Collision {
    //pub on_body: u64,
    pub mass: f64,
    pub vel: Vector3<f64>,
}

#[derive(PartialEq, Copy, Clone)]
pub struct SuspectedCollision {
    meter: f64,
}

#[derive(PartialEq, Copy, Clone)]
pub enum SuspectCollChange {
    Increase,
    Decrease,
    Stagnant,
}

impl Body {
    pub fn new(x: f64, y: f64, vx: f64, vy: f64, mass: f64) -> Body {
        Body {
            pos: Vector3::new(x, y, 0.0),
            vel: Vector3::new(vx, vy, 0.0),
            mass,
            class: BodyType::Massive,
            id: unsafe { ID_TABLE.take_new_id() },
            suspect_for_collision: HashMap::new(),
            phys_radius: Self::calculate_radius(mass)
        }
    }

    pub fn new_by_vec3(pos: Vector3<f64>, vel: Vector3<f64>, mass: f64, class: BodyType) -> Body {
        let id = unsafe { ID_TABLE.take_new_id() };
        Body {
            pos,
            vel,
            mass,
            class,
            id,
            suspect_for_collision: HashMap::new(),
            phys_radius: Self::calculate_radius(mass)
        }
    }

    pub fn calculate_radius(mass: f64) -> f64{
        use std::f64::consts::PI;
        (mass * BODY_DENSITY_VALUE / PI).sqrt()
    }

    pub fn get_id(&self) -> u64 {
        self.id
    }

    pub fn get_radius(&self) -> f64 {
        self.phys_radius
    }

    pub fn update_radius(&mut self) {
        self.phys_radius = Self::calculate_radius(self.mass)
    }

    pub fn check_for_collision<'a>(&self, bodies: &'a Vec<Body>) -> Option<&'a Body> {
        for (id, coll) in &self.suspect_for_collision {
            if coll.meter >= SUSPECT_COLLISION_THRESHOLD {
                for body_2 in bodies {
                    if *id == body_2.id {
                        return Some(&body_2);
                    }
                }
            }
        }
        None
    }

    pub fn suspect_collision(&mut self, delta_t: f64, body_id: u64, sus_change: SuspectCollChange) {
        if let Some(suspect) = self.suspect_for_collision.get_mut(&body_id) {
            //let Suspected { meter, .. } = suspect;
            suspect.meter += delta_t * sus_change.value();
            if suspect.meter < 0.0 {
                self.suspect_for_collision.remove(&body_id);
            }
        } else {
            self.suspect_for_collision.insert(
                body_id,
                SuspectedCollision {
                    meter: delta_t * sus_change.value(),
                },
            );
        }
    }
}

impl SuspectCollChange {
    pub fn value(&self) -> f64 {
        match self {
            Increase => 1.0,
            Decrease => -1.0,
            Stagnant => 0.0,
        }
    }
}

impl Clone for Body {
    fn clone(&self) -> Self {
        Body {
            pos: self.pos,
            vel: self.vel,
            mass: self.mass,
            class: self.class,
            id: self.id,
            suspect_for_collision: self
                .suspect_for_collision
                .iter()
                .map(|(id, coll)| (*id, coll.clone()))
                .collect::<HashMap<u64, SuspectedCollision>>(),
            phys_radius: self.phys_radius
        }
    }
}

pub fn split_task_length(task_size: usize, number_of_threads: usize) -> Vec<usize> {
    let sub_task = task_size / number_of_threads;
    let reminder = task_size % number_of_threads;
    let mut result = vec![sub_task; number_of_threads];
    for i in 0..reminder {
        result[i] += 1
    }
    result
}

// 8/3 -> 3 3 2 ; 7/4 -> 2 2 2 1
#[allow(dead_code)]
pub fn split_task<T>(objects: &mut Vec<T>, number_of_threads: usize) -> Vec<&mut [T]> {
    let mut tasks = Vec::new();
    let mut rem_task = objects as &mut [T];
    let rem = rem_task.len() % number_of_threads;
    let subtask = rem_task.len() / number_of_threads;
    for i in 0..number_of_threads - 1 {
        let mut sep = subtask;
        if i < rem {
            sep += 1;
        }
        let (tsk, remain) = rem_task.split_at_mut(sep);
        tasks.push(tsk);
        rem_task = remain;
    }
    tasks.push(rem_task);
    tasks
}
