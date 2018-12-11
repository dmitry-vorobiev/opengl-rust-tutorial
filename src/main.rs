extern crate sdl2;
extern crate gl;
extern crate image;
extern crate vec_2_10_10_10;
#[macro_use] extern crate failure;
#[macro_use] extern crate render_gl_derive;

pub mod render_gl;
pub mod resources;
mod triangle;
mod debug;

use render_gl::data::{f32_f32_f32, u2_u10_u10_u10_rev_float};

#[derive(VertexAttribPointers)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
struct Vertex {
    #[location = "0"]
    pos: f32_f32_f32,
    #[location = "1"]
    clr: u2_u10_u10_u10_rev_float,
}

fn main() {
    if let Err(e) = run() {
        println!("{}", debug::failure_to_string(e));
    }
}

fn run() -> Result<(), failure::Error> {
    use failure::err_msg;

    let sdl = sdl2::init().map_err(err_msg)?;
    let video_subsystem = sdl.video().map_err(err_msg)?;
    let gl_attr = video_subsystem.gl_attr();

    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 5);

    let window = video_subsystem
        .window("Game", 900, 700)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().map_err(err_msg)?;

    let gl = gl::Gl::load_with(
        |s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void
    );
    let mut viewport = render_gl::Viewport::for_window(900, 700);
    viewport.set_used(&gl);


    unsafe {
        gl.ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    use std::path::Path;
    use resources::Resources;

    let res = Resources::from_relative_exe_path(Path::new("assets"))?;
    let triangle = triangle::Triangle::new(&res, &gl)?;

    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            use sdl2::event::{Event, WindowEvent};
            match event {
                Event::Quit {..} => break 'main,
                Event::Window {win_event: WindowEvent::Resized(w, h), ..} => {
                    viewport.update_size(w, h);
                    viewport.set_used(&gl);
                },
                _ => {},
            }
        }
        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT);
        }
        triangle.render(&gl);
        window.gl_swap_window();
    }

    Ok(())
}
