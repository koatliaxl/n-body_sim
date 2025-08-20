use crate::{
    apply_collisions, begin_next_step, check_if_tasks_finished, update_world, Config, State, World,
};

pub fn predict(world: &World, state: &mut State, cfg: &Config, delta: f64) {
    //let mut predicted = world.clone();

    /*let State {
        selected,
        predicion
                ..
    } = state;*/

    let lock = world.bodies.lock();
    let bodies = lock.expect("lock must be acquired on original bodies");
    let mut prediction = &mut state.prediction.state;
    let mut pred_state = prediction
        .bodies
        .lock()
        .expect("lock must be acquired on bodies copy");
    pred_state.clear();
    for body in bodies.iter() {
        pred_state.push(body.clone());
    }

    //let mut step_done = false;
    for _ in cfg.prediction_steps {
        begin_next_step(&mut prediction, delta, state);
        while state.received < state.workers.len() {
            check_if_tasks_finished(state)
        }
        update_world(&mut prediction);
        apply_collisions(&mut prediction);
        let mut pred_state = prediction
            .bodies
            .lock()
            .expect("lock must be acquired on bodies copy");
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
                state.prediction.trajectory.push(body.pos)
            }
        }
    }
}
