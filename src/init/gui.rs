use mat_vec::Vector3;
use n_body_sim::gui::{Label, RootGIE};

pub fn init_gui(window_size: (i32, i32)) -> RootGIE {
    let mut gui = RootGIE::new();
    let text_draw = crate::draw::draw_text;
    let body_pos = Label::new((0, 0), "body_pos_label".to_string(), "", 0.5, text_draw);
    gui.add_gie(body_pos);
    let body_mass = Label::new((0, 0), "body_mass_label".to_string(), "", 0.4, text_draw);
    gui.add_gie(body_mass);
    let (w, _h) = window_size;
    let mut ups_counter = Label::new((w - 60, 20), "ups_counter".to_string(), "", 0.4, text_draw);
    let mut fps_counter = Label::new((w - 115, 20), "fps_counter".to_string(), "", 0.4, text_draw);
    let color_1 = Vector3::new(0.2, 0.6, 0.6);
    ups_counter.set_color(color_1);
    fps_counter.set_color(color_1);
    gui.add_gie(ups_counter);
    gui.add_gie(fps_counter);
    let lu_duration = Label::new((10, 20), "upd_average".to_string(), "", 0.4, text_draw);
    gui.add_gie(lu_duration);
    gui
}
