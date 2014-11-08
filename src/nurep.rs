extern crate native;
extern crate sdl2;
extern crate serialize;

use serialize::json;
use std::io;
use std::os;

mod drawing;
mod state;

fn print_usage() {
    println!("Usage: nurep <data_path>");
}

fn main() {
    let args: Vec<String> = os::args();
    if args.len() != 2 {
        print_usage();
        return;
    }
    let data_path = args[1].as_slice();

    let json = io::File::open(&Path::new(data_path)).read_to_string().unwrap();
    let state: state::State = json::decode(json.as_slice()).unwrap();

    sdl2::init(sdl2::INIT_VIDEO);

    let window = match sdl2::video::Window::new(
            "nurep",
            sdl2::video::PosCentered,
            sdl2::video::PosCentered,
            1000,
            1000,
            sdl2::video::OPENGL) {
        Ok(window) => window,
        Err(err) => panic!(format!("failed to create window: {}", err))
    };

    let renderer = match sdl2::render::Renderer::from_window(
            window,
            sdl2::render::DriverAuto,
            sdl2::render::ACCELERATED) {
        Ok(renderer) => renderer,
        Err(err) => panic!(format!("failed to create renderer: {}", err))
    };

    let _ = draw(&renderer, &state);
    renderer.present();

    'main : loop {
        'event : loop {
            match sdl2::event::poll_event() {
                sdl2::event::QuitEvent(_) => break 'main,
                sdl2::event::KeyDownEvent(_, _, key, _, _) => {
                    if key == sdl2::keycode::EscapeKey {
                        break 'main
                    }
                },
                sdl2::event::NoEvent => break 'event,
                _ => {}
            }
        }
    }

    sdl2::quit();
}

struct State {
    pub circle_position: (i32, i32),
}

#[must_use]
fn draw(renderer: &sdl2::render::Renderer, state: &state::State) -> sdl2::SdlResult<()> {
    let (w, h) = state.cluster.dimensions;
    let w_factor: f64 = 1000f64 / (w as f64);
    let h_factor: f64 = 1000f64 / (h as f64);

    try!(renderer.set_draw_color(sdl2::pixels::RGB(0, 0, 0)));
    try!(renderer.clear());

    for planet in state.cluster.planets.iter() {
        let (x, y) = planet.position;
        try!(drawing::draw_circle(
            renderer,
            ((x as f64 * w_factor) as i32 - (w/4), 1000i32 - ((y as f64 * h_factor) as i32 - (h/4))),
            5,
            sdl2::pixels::RGB(255, 0, 0)));
    }

    Ok(())
}
