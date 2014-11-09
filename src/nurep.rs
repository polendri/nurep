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
    let game: state::Game = json::decode(json.as_slice()).unwrap();

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

    let state = State { turn: 1, game: game };
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
    pub turn: i32,
    pub game: state::Game,
}

fn pick_color(owner_id: i32) -> sdl2::pixels::Color {
    match owner_id {
        0  => sdl2::pixels::RGB(0x50, 0x50, 0x50),
        1  => sdl2::pixels::RGB(0xff, 0x00, 0x00),
        2  => sdl2::pixels::RGB(0x00, 0xff, 0x00),
        3  => sdl2::pixels::RGB(0x00, 0x00, 0xff),
        4  => sdl2::pixels::RGB(0xff, 0xff, 0x00),
        5  => sdl2::pixels::RGB(0x00, 0xff, 0xff),
        6  => sdl2::pixels::RGB(0xff, 0x00, 0xff),
        7  => sdl2::pixels::RGB(0xc0, 0x80, 0x00),
        8  => sdl2::pixels::RGB(0x00, 0xc0, 0x80),
        9  => sdl2::pixels::RGB(0xc0, 0x00, 0x80),
        10 => sdl2::pixels::RGB(0x80, 0xc0, 0x00),
        11 => sdl2::pixels::RGB(0x00, 0x80, 0xc0),
        _  => sdl2::pixels::RGB(0xff, 0xff, 0xff),
    }
}

#[must_use]
fn draw(renderer: &sdl2::render::Renderer, state: &State) -> sdl2::SdlResult<()> {
    let (w, h) = state.game.cluster.dimensions;
    let w_factor: f64 = 1000f64 / (w as f64);
    let h_factor: f64 = 1000f64 / (h as f64);

    try!(renderer.set_draw_color(sdl2::pixels::RGB(0, 0, 0)));
    try!(renderer.clear());

    for planet in state.game.cluster.planets.iter() {
        let owner = *state.game.planet_to_owners.get(&planet.id).unwrap().get(&9).unwrap();
        let color = pick_color(owner);
        let (x, y) = planet.position;
        try!(drawing::draw_circle(
            renderer,
            ((x as f64 * w_factor) as i32 - (w/2), 1000i32 - ((y as f64 * h_factor) as i32 - (h/2))),
            5,
            color));
    }

    Ok(())
}
