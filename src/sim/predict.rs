use crate::state_and_cfg::Prediction;
use crate::{
    apply_collisions, begin_next_step, check_if_tasks_finished, update_world, Config, State, World,
};
use n_body_sim::BodyType::Removed;

pub fn predict(world: &World, state: &mut State, cfg: &Config, delta: f64) {
    //let selected_body;
    if state.prediction.selected_ceased_to_exist_on >= 0 {
        return;
    }
    if state.prediction.history.is_empty() {
        let State {
            prediction:
                Prediction {
                    trajectory,
                    state: predicted,
                    ..
                },
            selected,
            ..
        } = state;
        let lock = world.bodies.lock();
        let bodies = lock.expect("lock must be acquired on original bodies");
        let mut pred_state = predicted
            .bodies
            .lock()
            .expect("lock must be acquired on bodies copy");
        pred_state.clear();
        trajectory.clear();
        for body in bodies.iter() {
            pred_state.push(body.clone());
            if body.get_id() == *selected as u64 {
                //selected_body = body.clone();
                trajectory.push_back(body.pos);
            }
        }
    }
    let mut early_exit = false;
    for i in state.prediction.history.len()..cfg.prediction_steps {
        //println!("entered pred. loop");
        begin_next_step(&state.prediction.state, delta, state, true);
        while state.prediction.task_done_count < state.workers.len() {
            check_if_tasks_finished(state, true);
        }
        //println!("tasks finished");
        state.prediction.task_done_count = 0;
        {}
        let State {
            ref selected,
            prediction:
                Prediction {
                    trajectory,
                    state: ref predicted,
                    history,
                    ..
                },
            ..
        } = state;
        //println!("before update");
        update_world(predicted);
        //println!("after update");
        let mut bodies = Vec::new();
        {
            let pred_state = predicted
                .bodies
                .lock()
                .expect("lock must be acquired on bodies copy");
            'inner: for body in pred_state.iter() {
                bodies.push(body.clone());
                if body.get_id() == *selected as u64 {
                    trajectory.push_back(body.pos);
                    if body.class == Removed {
                        state.prediction.selected_ceased_to_exist_on = i as isize;
                        early_exit = true;
                        //break 'inner;
                    }
                    //break 'inner;
                }
            }
        }
        //println!("before applying collisions");
        apply_collisions(predicted);
        //println!("after applying collisions");
        history.push_back(bodies);
        if early_exit {
            return;
        }
    }
}
