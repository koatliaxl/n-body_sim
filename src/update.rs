use crate::draw::BODY_GFX_SCALE;
use crate::handle_input::select::*;
use crate::sim::World;
use crate::state_and_cfg::State;
use crate::Statistic;
use core::time::Duration;
use mat_vec::Vector3;
use n_body_sim::gui::{Label, RootGIE};
use n_body_sim::support::in_pixels;

pub fn update_gui_state(
    state: &State,
    world: &World,
    window_size: (i32, i32),
    gui: &mut RootGIE,
    statistic: &Statistic,
) {
    if state.selected > -1 {
        let (proj, view) = calc_matrices(state, window_size);
        for body in world.bodies.lock().expect("Lock must be acquired").iter() {
            if state.selected == body.get_id() as i64 {
                let pos_on_scr = calc_body_pos_on_screen(window_size, &view, &proj, &body);
                let radius = BODY_GFX_SCALE * body.get_radius() as f32;
                let (r_px, _) = in_pixels((radius, 0.0), window_size);
                let mut info_offset = Vector3::new(r_px as f32, 0.0, 0.0);
                info_offset /= state.view_scale;
                //let pos_in_pixels = conv_coord_base(pos_on_scr + info_offset, window_size);
                let (x, y, _) = (pos_on_scr + info_offset).get_components();
                update_selected_info(gui, (x as i32, y as i32), body);
            }
        }
    } else {
        gui.get_gie("body_pos_label")
            .unwrap()
            .get_base_mut()
            .visible = false;
        gui.get_gie("body_mass_label")
            .unwrap()
            .get_base_mut()
            .visible = false;
    }

    gui.get_gie("ups_counter")
        .unwrap()
        .get_type()
        .downcast_mut::<Label>()
        .expect("GIE must be downcast")
        .change_text(format!("UPS: {}", state.ups));
    gui.get_gie("fps_counter")
        .unwrap()
        .get_type()
        .downcast_mut::<Label>()
        .expect("GIE must be downcast")
        .change_text(format!("FPS: {}", state.fps));

    let mut sum = Duration::from_secs(0);
    for m in &statistic.upd_measure_history {
        sum += *m;
    }
    let average = (sum.as_secs_f32() * 1000.0) / statistic.upd_measure_num as f32;
    gui.get_gie("upd_average")
        .unwrap()
        .get_type()
        .downcast_mut::<Label>()
        .expect("GIE must be downcast")
        .change_text(format!(
            "Average of {} updates (millis): {:.5}",
            statistic.upd_measure_num, average
        ));
}
