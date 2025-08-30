use crate::state_and_cfg::{Prediction, State};
use n_body_sim::Body;

impl State {
    pub fn progress_to_next_step(&mut self) -> Option<Vec<Body>> {
        self.prediction.trajectory.pop_front();
        if self.prediction.selected_ceased_to_exist_on > 0 {
            self.prediction.selected_ceased_to_exist_on -= 1;
        } else if self.prediction.history.is_empty() {
            self.prediction.selected_ceased_to_exist_on = -1;
            self.selected = -1;
        }
        self.prediction.history.pop_front()
    }
}

impl Prediction {
    pub fn devalidate_history(&mut self) {
        self.history.clear();
        //self.trajectory.clear();
        self.selected_ceased_to_exist_on = -1;
    }
}
