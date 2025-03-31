extern crate core;

use mat_vec::Vector3;

pub const SIZE_OF_GL_FLOAT: isize = std::mem::size_of::<gl::types::GLfloat>() as isize;

pub mod gl {
    include!("../gl/bindings.rs");
}

#[cfg(test)]
mod tests;

pub struct Object {
    pub pos: Vector3<f64>,
    pub vel: Vector3<f64>,
    pub mass: f64,
    pub class: ObjectType,
}

#[derive(PartialEq, Copy, Clone)]
pub enum ObjectType {
    Removed,
    Massive,
    Light,
}

impl Clone for Object {
    fn clone(&self) -> Self {
        Object {
            pos: self.pos,
            vel: self.vel,
            mass: self.mass,
            class: self.class,
        }
    }
}

pub fn split_task_length(task_size: usize, number_of_threads: usize) -> Vec<usize> {
    let mut sub_task = task_size / number_of_threads;
    let reminder = task_size % number_of_threads;
    let mut result = vec![sub_task; number_of_threads];
    for i in 0..reminder {
        result[i] += 1
    }
    result
}

#[allow(dead_code)]
pub fn split_task<T>(objects: &mut Vec<T>, number_of_threads: usize) -> Vec<&mut [T]> {
    let mut tasks = Vec::new();
    let mut rem_task = objects as &mut [T];
    let rem = rem_task.len() % number_of_threads;
    // 8/3 -> 3 3 2 ; 7/4 -> 2 2 2 1
    let mut subtask = rem_task.len() / number_of_threads;
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
