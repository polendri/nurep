extern crate native;
extern crate sdl2;
extern crate serialize;

use serialize::json;
use std::cmp;
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
            0,
            0,
            sdl2::video::FULLSCREEN_DESKTOP) {
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

    // TODO: Somehow know the right display index instead of just picking 0
    let (screen_width, screen_height) = match sdl2::video::get_current_display_mode(0) {
        Ok(display_mode) => (display_mode.w as i32, display_mode.h as i32),
        Err(err) => panic!(format!("failed to retrieve display size: {}", err))
    };
    let draw_size = cmp::min(screen_width, screen_height);
    let scale_factor = 0.95 * (draw_size as f64) / (cmp::max(game.cluster.dimensions.val0(), game.cluster.dimensions.val1()) as f64);


    let mut state = State {
        turn: 1,
        game: game,
        draw_size: draw_size,
        draw_offsets: ((screen_width - draw_size) / 2, (screen_height - draw_size) / 2),
        scale_factor: scale_factor,
    };

    'main : loop {
        if state.turn > state.game.num_turns {
            break;
        }

        let start_time = sdl2::timer::get_ticks();

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

        let _ = draw(&renderer, &state);
        renderer.present();

        state.turn += 1;
        let elapsed = sdl2::timer::get_ticks() - start_time;
        if elapsed < 250 {
            sdl2::timer::delay(250 - elapsed);
        }
    }

    sdl2::quit();
}

struct State {
    pub turn: i32,
    pub game: state::Game,
    pub draw_size: i32,
    pub draw_offsets: (i32, i32),
    pub scale_factor: f64,
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

/// Transforms a coordinate pair from game coordinates to screen coordinates.
fn transform_coord(state: &State, coord: (i32, i32)) -> (i32, i32) {
    let (x, y) = coord;
    let (scaled_x, scaled_y) = (((x as f64) * state.scale_factor) as i32, ((y as f64 * state.scale_factor)) as i32);
    let (offset_x, offset_y) = state.draw_offsets;
    (scaled_x + offset_x, scaled_y + offset_y)
}

#[must_use]
fn draw(renderer: &sdl2::render::Renderer, state: &State) -> sdl2::SdlResult<()> {
    let radius = state.draw_size / 200;
    try!(renderer.set_draw_color(sdl2::pixels::RGB(0, 0, 0)));
    try!(renderer.clear());

    for planet in state.game.cluster.planets.iter() {
        let owner = match state.game.planet_to_owners.get(&planet.id) {
            Some(turn_to_owner) => match turn_to_owner.get(&state.turn) {
                Some(x) => *x,
                None => -1,
            },
            None => -1,
        };
        let color = pick_color(owner);
        let (x, y) = transform_coord(state, planet.position);
        try!(drawing::draw_circle(renderer, (x, y), radius, color));
    }

    Ok(())
}
