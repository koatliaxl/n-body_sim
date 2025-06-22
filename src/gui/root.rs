use crate::gui::GIE;
use crate::GlData;
use core::option::Option;

pub struct RootGIE {
    contain: Vec<Box<dyn GIE>>,
}

impl RootGIE {
    pub fn new() -> RootGIE {
        RootGIE {
            contain: Vec::new(),
        }
    }

    pub fn add_gie<Gie>(&mut self, gie: Gie)
    where
        Gie: GIE + 'static,
    {
        self.contain.push(Box::new(gie))
    }

    pub fn draw(&self, gl_data: &GlData) {
        for gie in &self.contain {
            gie.draw(gl_data, gie.get_base())
        }
    }

    pub fn get_gie(&mut self, name: &str) -> Option<&mut dyn GIE> {
        for gie in &mut self.contain {
            return gie.get_gie(name);
        }
        None
    }
}

/*impl GIE for RootGIE {
    fn get_base(&self) -> &GieBase {
        &self.contain
    }
}*/
