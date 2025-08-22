use crate::state_and_cfg::Prediction;
use crate::{
    apply_collisions, begin_next_step, check_if_tasks_finished, update_world, Config, State, World,
};

pub fn predict(world: &World, state: &mut State, cfg: &Config, delta: f64) {
    //let mut predicted = world.clone();

    let State {
        selected,
        prediction,
        ..
    } = &state;
    let Prediction {
        trajectory,
        state: predicted,
        ..
    } = prediction;

    let lock = world.bodies.lock();
    let bodies = lock.expect("lock must be acquired on original bodies");
    //let mut prediction = &mut state.prediction.state;
    {
        let mut pred_state = predicted
            .bodies
            .lock()
            .expect("lock must be acquired on bodies copy");
        pred_state.clear();
        for body in bodies.iter() {
            pred_state.push(body.clone());
        }
    }

    //let mut step_done = false;
    for _ in 0..cfg.prediction_steps {
        begin_next_step(predicted, delta, state, true);
        while state.received < state.workers.len() {
            check_if_tasks_finished(state)
        }
        update_world(predicted);
        apply_collisions(predicted);
        let mut pred_state = predicted
            .bodies
            .lock()
            .expect("lock must be acquired on bodies copy");
        trajectory.clear();
        for body in pred_state.iter() {
            if body.get_id() == state.selected as u64 {
                /*if i>0 {
                    let last_pos = if let Some(v) = state.prediction.trajectory.last() {
                        *v
                    } else {
                        panic!("There is must be last position if prediction step > 1")
                    };
                    let diff = body.pos - last_pos;
                }*/
                trajectory.push(body.pos)
            }
        }
    }
}
