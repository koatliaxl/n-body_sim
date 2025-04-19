use crate::gl;
use crate::gl::types::GLenum;
use std::io::Read;

pub unsafe fn gen_shader_from_file(path: &str, shader_type: GLenum, name: &str) -> u32 {
    let mut source_file = std::fs::File::open(path).expect("Fail to open shader src file");
    let mut source = String::new();
    source_file
        .read_to_string(&mut source)
        .expect("Fail to read shader src file");
    let shader_id = gl::CreateShader(shader_type);
    gl::ShaderSource(
        shader_id, /* Rustfmt force vertical formatting */
        1,
        &(source.as_ptr() as *const i8),
        &(source.len() as i32),
    );
    gl::CompileShader(shader_id);
    check_compile_status(shader_id, name, false);
    shader_id
}

unsafe fn check_compile_status(shader_id: u32, shader_name: &str, print_source: bool) {
    let mut success = 0;
    gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut success);
    println!("\"{1:}\" compile status: {0:}", success, shader_name);
    let mut log = ['\0' as u8; 512];
    if success == 0 {
        gl::GetShaderInfoLog(
            shader_id,
            log.len() as i32,
            &mut 0,
            log.as_mut_ptr() as *mut i8,
        );
        println!("{}", std::str::from_utf8_unchecked(&log));
    }
    if print_source {
        let mut length = 0;
        gl::GetShaderiv(shader_id, gl::SHADER_SOURCE_LENGTH, &mut length);
        println!("source length: {}", length);
        gl::GetShaderSource(
            shader_id,
            log.len() as i32,
            &mut 0,
            log.as_mut_ptr() as *mut i8,
        );
        println!("source:\n\n{}\n", std::str::from_utf8_unchecked(&log));
    }
}

pub unsafe fn gen_shader_program(
    vertex_shader_id: u32,
    fragment_shader_id: u32,
    name: &str,
) -> u32 {
    let shader_program_id = gl::CreateProgram();
    gl::AttachShader(shader_program_id, vertex_shader_id);
    gl::AttachShader(shader_program_id, fragment_shader_id);
    gl::LinkProgram(shader_program_id);

    let mut success = 0;
    gl::GetProgramiv(shader_program_id, gl::LINK_STATUS, &mut success);
    println!("\"{1:}\" link status: {}", success, name);
    if success == 0 {
        let mut log = [' ' as u8; 512];
        let null_mut = &mut 0 as *mut _;
        gl::GetProgramInfoLog(
            shader_program_id,
            log.len() as i32,
            null_mut,
            log.as_mut_ptr() as *mut _,
        );
        println!("{}", std::str::from_utf8_unchecked(&log));
    }
    shader_program_id
}

#[allow(dead_code)]
pub unsafe fn gen_geometry_shader_program(
    vertex_shader_id: u32,
    fragment_shader_id: u32,
    geometry_shader_id: u32,
    name: &str,
) -> u32 {
    let shader_program_id = gl::CreateProgram();
    gl::AttachShader(shader_program_id, vertex_shader_id);
    gl::AttachShader(shader_program_id, fragment_shader_id);
    gl::AttachShader(shader_program_id, geometry_shader_id);
    gl::LinkProgram(shader_program_id);

    let mut success = 0;
    gl::GetProgramiv(shader_program_id, gl::LINK_STATUS, &mut success);
    println!("\"{1:}\" link status: {}", success, name);
    if success == 0 {
        let mut log = [' ' as u8; 512];
        let null_mut = &mut 0 as *mut _;
        gl::GetProgramInfoLog(
            shader_program_id,
            log.len() as i32,
            null_mut,
            log.as_mut_ptr() as *mut _,
        );
        println!("{}", std::str::from_utf8_unchecked(&log));
    }
    shader_program_id
}
