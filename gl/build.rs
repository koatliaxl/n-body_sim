use gl_generator::{Api, Fallbacks, GlobalGenerator, Profile, Registry};
use std::fs::File;

fn main() {
    if let Err(_) = File::open("gl/bindings.rs") {
        let mut file = File::create("gl/bindings.rs").unwrap();
        Registry::new(Api::Gl, (3, 3), Profile::Core, Fallbacks::All, [])
            .write_bindings(GlobalGenerator, &mut file)
            .unwrap();
    }
    println!("cargo:rerun-if-changed=./gl/bindings.rs");
}
