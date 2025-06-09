use super::{GieBase, GIE};
use crate::GlData;
use std::any::Any;
//use crate::gui::MetaType::Compound;

pub struct Label {
    base: GieBase,
    text: String,
    text_size: f32,
    draw_function: unsafe fn(gl_res: &GlData, text: &str, pos: (i32, i32), scale: f32),
}

impl Label {
    pub fn new(
        pos: (i32, i32),
        name: String,
        text: &str,
        text_size: f32,
        draw_function: unsafe fn(gl_res: &GlData, text: &str, pos: (i32, i32), scale: f32),
    ) -> Label {
        let base = GieBase::new(pos, name, false);
        Label {
            base,
            text: text.to_string(),
            text_size,
            draw_function,
        }
    }

    pub fn change_text(&mut self, text: String) {
        self.text = text
    }

    pub fn get_text(&self) -> &str {
        &self.text
    }
}

impl GIE for Label {
    fn draw(&self, gl_data: &GlData, base: &GieBase) {
        unsafe {
            (self.draw_function)(
                gl_data,
                &self.text,
                (base.pos.x(), base.pos.y()),
                self.text_size,
            )
        }
    }

    fn get_base(&self) -> &GieBase {
        &self.base
    }

    fn get_base_mut(&mut self) -> &mut GieBase {
        &mut self.base
    }

    fn get_gie(&mut self, name: &str) -> Option<&mut dyn GIE> {
        /*if self.get_base().name.as_str() == name {
            return Some(self);
        }
        if let Compound { ref contain } = self.get_base().meta_type {
            for gie in contain {
                return gie.get_gie(name);
            }
        }
        None*/
        //let (base, gie_type) = (&mut self.base, &mut self);
        if self.base.name == name {
            return Some(self);
        }
        self.base.get_gie(name)
        //base.get_gie(name, *gie_type)
    }

    fn get_type(&mut self) -> &mut dyn Any {
        self
    }
}
