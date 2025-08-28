use crate::state_and_cfg::Prediction;
use crate::{
    apply_collisions, begin_next_step, check_if_tasks_finished, update_world, Config, State, World,
};
use n_body_sim::BodyType::Removed;

pub fn predict(world: &World, state: &mut State, cfg: &Config, delta: f64) {
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
    //let selected_body;
    {
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
                trajectory.push(body.pos);
            }
        }
    }
    /*if !state.update_processed {
        while state.task_done_count < state.workers.len() {
            check_if_tasks_finished(state);
        }
    }*/
    let mut early_exit = false;
    for _ in 0..cfg.prediction_steps {
        //println!("inside pred. loop");
        begin_next_step(&state.prediction.state, delta, state, true);
        //println!("given tasks");
        while state.prediction.task_done_count < state.workers.len() {
            check_if_tasks_finished(state, true);
        }
        //println!("tasks finished");
        state.prediction.task_done_count = 0;
        let State {
            ref selected,
            prediction:
                Prediction {
                    trajectory,
                    state: ref predicted,
                    ..
                },
            ..
        } = state;
        update_world(predicted);
        //println!("after update");
        {
            let pred_state = predicted
                .bodies
                .lock()
                .expect("lock must be acquired on bodies copy");
            'inner: for body in pred_state.iter() {
                if body.get_id() == *selected as u64 {
                    if body.class == Removed {
                        //apply_collisions(predicted);
                        early_exit = true;
                        break 'inner;
                    }
                    /*if i>0 {
                        let last_pos = if let Some(v) = state.prediction.trajectory.last() {
                            *v
                        } else {
                            panic!("There is must be last position if prediction step > 1")
                        };
                        let diff = body.pos - last_pos;
                    }*/
                    trajectory.push(body.pos);
                    break 'inner;
                }
            }
        }
        apply_collisions(predicted);
        if early_exit {
            return;
        }
    }
}
