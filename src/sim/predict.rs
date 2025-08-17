use crate::{apply_collisions, begin_next_step, check_if_tasks_finished, Config, State, update_world, World};

pub fn predict(world: &World, state: &mut State, cfg: &Config, delta: f64) {
    let mut predicted = world.clone();

    //let mut step_done = false;
    for _ in cfg.prediction_steps {
        begin_next_step(&mut predicted, delta,state);
        while state.received < state.workers.len() {
            check_if_tasks_finished(state)
        }
        update_world(&mut predicted);
        apply_collisions(&mut predicted);

        state.prediction.push()
    }
}