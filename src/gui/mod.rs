use core::option::Option;
use std::any::Any;
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

    /*pub fn change_pos(&mut self, pos: (i32, i32)) {

    }*/

    fn get_gie<'a>(
        &'a mut self,
        name: &str, /*, gie_type: &'a mut dyn GIE*/
    ) -> Option<&'a mut dyn GIE> {
        /*if self.name.as_str() == name {
            return Some(gie_type);
        }*/
        if let Compound { ref mut contain } = self.meta_type {
            for gie in contain {
                //return gie.get_base_mut().get_gie(name, self);
                return gie.get_gie(name);
            }
        }
        None
    }
}

pub trait GIE {
    fn draw(&self, gl_data: &GlData, base: &GieBase) {
        if let Compound { contain } = &base.meta_type {
            for gie in contain {
                gie.draw(gl_data, gie.get_base())
            }
        }
    }

    fn get_base(&self) -> &GieBase;

    fn get_base_mut(&mut self) -> &mut GieBase;

    fn change_pos(&mut self, pos: (i32, i32)) {
        *self.get_base_mut().pos.x_mut() = pos.0;
        *self.get_base_mut().pos.y_mut() = pos.1
    }

    fn get_gie(&mut self, name: &str) -> Option<&mut dyn GIE>; /*{
                                                                   let base = self.get_base();
                                                                   base.get_gie(name, self)
                                                               }*/
    /*where
    Self: Sized,*/
    /*{
        if self.get_base().name.as_str() == name {
            return Some(self); // this doesn't work
        }
        if let Compound { ref contain } = self.get_base().meta_type {
            for gie in contain {
                return gie.get_gie(name);
            }
        }
        None
    }*/

    fn get_type(&mut self) -> &mut dyn Any;
}
