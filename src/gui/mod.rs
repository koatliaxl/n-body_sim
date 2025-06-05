use crate::GlData;
use mat_vec::Vector3;

pub struct RootGIE {
    //contain: Vec<GuiElem>,
}

pub struct GuiElem {
    pos: Vector3<i32>,
    name: String,
    visible: bool,
    gui_type: Box<dyn GieType>,
    meta_type: MetaType,
}

pub enum MetaType {
    Simple,
    Compound { childs: Vec<GuiElem> },
}

pub trait GieType {
    fn draw(&self, gl_data: &GlData, base: &GuiElem);
}

pub struct Label {
    base: GuiElem,
    text: String,
    text_size: f32,
    draw_function: fn(gl_res: &GlData, text: &str, pos: (i32, i32), scale: f32),
}

impl GieType for Label {
    fn draw(&self, gl_data: &GlData, base: &GuiElem) {
        (self.draw_function)(
            gl_data,
            &self.text,
            (base.pos.x(), base.pos.y()),
            self.text_size,
        )
    }
}
