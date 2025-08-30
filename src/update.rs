use crate::draw::BODY_GFX_SCALE;
use crate::handle_input::select::*;
use crate::sim::World;
use crate::state_and_cfg::State;
use mat_vec::Vector3;
use n_body_sim::gui::RootGIE;
use n_body_sim::support::in_pixels;

pub fn update_gui(state: &State, world: &World, window_size: (i32, i32), gui: &mut RootGIE) {
    if state.selected > -1 {
        let (proj, view) = calc_matrices(state, window_size);
        for body in world.bodies.lock().expect("Lock must be acquired").iter() {
            if state.selected == body.get_id() as i64 {
                let pos_on_scr = calc_body_pos_on_screen(window_size, &view, &proj, &body);
                let radius = BODY_GFX_SCALE * body.get_radius() as f32;
                let (r_px, _) = in_pixels((radius, 0.0), window_size);
                let mut info_offset = Vector3::new(r_px as f32, 0.0, 0.0);
                info_offset /= state.view_scale;
                let pos_in_pixels = fix_screen_pos(pos_on_scr + info_offset, window_size);
                update_selected_info(gui, pos_in_pixels, body);
            }
        }
    } else {
        gui.get_gie("body_pos_text").unwrap().get_base_mut().visible = false;
        gui.get_gie("body_mass_text")
            .unwrap()
            .get_base_mut()
            .visible = false;
    }
}
