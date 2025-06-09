use n_body_sim::gui::{Label, RootGIE};

pub fn init_gui() -> RootGIE {
    let mut gui = RootGIE::new();
    let text_draw = crate::draw::draw_text;
    let body_pos = Label::new((0, 0), "body_pos_text".to_string(), "", 1.0, text_draw);
    gui.add_gie(body_pos);
    gui
}
