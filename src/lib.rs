extern crate core;

use mat_vec::Vector3;

pub use support::id_table::ObjectIdTable;

pub const SIZE_OF_GL_FLOAT: isize = std::mem::size_of::<gl::types::GLfloat>() as isize;

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
    pub class: BodyType,
    id: u64,
}

#[derive(PartialEq, Copy, Clone)]
pub enum BodyType {
    Removed,
    Massive,
    Light,
}

impl Body {
    pub fn new(x: f64, y: f64, vx: f64, vy: f64, mass: f64) -> Body {
        Body {
            pos: Vector3::new(x, y, 0.0),
            vel: Vector3::new(vx, vy, 0.0),
            mass,
            class: BodyType::Massive,
            id: unsafe { ID_TABLE.take_new_id() },
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
        }
    }

    pub fn get_id(&self) -> u64 {
        self.id
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
