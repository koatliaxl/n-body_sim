use core::marker::Sized;
use core::option::Option;
//use core::option::Option;
use crate::gui::MetaType::{Compound, Single};
use crate::GlData;
use mat_vec::Vector3;

pub use label::Label;
pub use root::RootGIE;

pub mod label;
pub mod root;

pub struct GieBase {
    pos: Vector3<i32>,
    name: String,
    pub visible: bool,
    //gui_type: Box<dyn GieType>,
    pub meta_type: MetaType,
}

pub enum MetaType {
    Single,
    Compound { contain: Vec<Box<dyn GIE>> },
}

impl GieBase {
    fn new(pos: (i32, i32), name: String, is_container: bool) -> GieBase {
        let meta_type = match is_container {
            true => Compound {
                contain: Vec::new(),
            },
            false => Single,
        };
        GieBase {
            pos: Vector3::new(pos.0, pos.1, 0),
            name,
            visible: false,
            meta_type,
        }
    }

    /*fn change_pos(pos: (i32, i32)) {

    }*/
}

pub trait GIE {
    fn draw(&self, gl_data: &GlData, base: &GieBase) {
        if let Compound { contain } = &base.meta_type {
            for gie in contain {
                gie.draw(gl_data, gie.get_base())
            }
        }
    }

    fn get_base(&mut self) -> &mut GieBase;

    fn change_pos(&mut self, pos: (i32, i32)) {
        *self.get_base().pos.x_mut() = pos.0;
        *self.get_base().pos.y_mut() = pos.1
    }

    fn get_gie(&mut self, name: &str) -> Option<Box<&mut dyn GIE>>
    where
        Self: GIE,
    {
        if self.get_base().name.as_str() == name {
            return Some(Box::new(self));
        }
        if let Compound { contain } = self.get_base().meta_type {
            for gie in contain {
                return gie.get_gie(name);
            }
        }
        None
    }
}
