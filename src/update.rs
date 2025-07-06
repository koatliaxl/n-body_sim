use crate::handle_input::select::*;
use crate::sim::World;
use crate::state_and_cfg::State;
use n_body_sim::gui::{Label, RootGIE, GIE};

pub fn update_gui(state: &State, world: &World, window_size: (i32, i32), gui: &mut RootGIE) {
    if state.selected > -1 {
        let (proj, view) = calc_matrices(state, window_size);
        for body in world.bodies.lock().expect("Lock must be acquired").iter() {
            if state.selected == body.get_id() as i64 {
                let pos_on_scr = calc_body_pos_on_screen(window_size, &view, &proj, &body);
                //println!("{},{}", pos_on_scr.x(), pos_on_scr.y());
                update_coord_label(gui, "body_pos_text", pos_on_scr, window_size, body);
            }
        }
    } else {
        let pos_label = gui.get_gie("body_pos_text").unwrap();
        pos_label
            .get_type()
            .downcast_mut::<Label>()
            .expect("failed to downcast GIE as Label")
            .get_base_mut()
            .visible = false
    }
}
