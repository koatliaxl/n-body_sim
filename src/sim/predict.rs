use n_body_sim::BodyType::Removed;
use crate::state_and_cfg::Prediction;
use crate::{
    apply_collisions, begin_next_step, check_if_tasks_finished, update_world, Config, State, World,
};

pub fn predict(world: &World, state: &mut State, cfg: &Config, delta: f64) {
    let State {
        prediction: Prediction {
            trajectory,
            state: predicted, ..
        },
        ..
    } =state;
    /*let Prediction {
        state: predicted,
        ..
    } = prediction;*/

    let lock = world.bodies.lock();
    let bodies = lock.expect("lock must be acquired on original bodies");
    {
        let mut pred_state = predicted
            .bodies
            .lock()
            .expect("lock must be acquired on bodies copy");
        pred_state.clear();
        for body in bodies.iter() {
            pred_state.push(body.clone());
        }
        trajectory.clear();
    }

    //let mut step_done = false;
    'outer: for _ in 0..cfg.prediction_steps {
        println!("inside prediction loop");
        begin_next_step(&state.prediction.state, delta, state, true);
        while state.received < state.workers.len() {
            check_if_tasks_finished(state);
            //println!("inside check finish task loop");
        }
        state.received = 0;

        let State {
            ref selected,
            prediction,
            ..
        } = state;
        let Prediction {
            trajectory,
            state: ref predicted,
            ..
        } = prediction;

        println!("before update");
        update_world(predicted);

        println!("update and apply collisions done");
        {
            let pred_state = predicted
                .bodies
                .lock()
                .expect("lock must be acquired on bodies copy");
            println!("lock acquired");
            'inner: for body in pred_state.iter() {
                if body.get_id() == *selected as u64 {
                    if body.class == Removed {
                        //apply_collisions(predicted);
                        break 'inner
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
                    println!("trajectory len: {}", trajectory.len());
                    break 'inner
                }
            }
        }
        apply_collisions(predicted);
        println!("applied collisions");
    }
    println!("exited from prediction loop")
}
