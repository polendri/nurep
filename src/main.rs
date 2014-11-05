extern crate sdl2;
extern crate native;

use drawing;

pub fn main() {
    sdl2::init(sdl2::INIT_VIDEO);

    let window = match sdl2::video::Window::new(
            "rust-sdl2 demo: Video",
            sdl2::video::PosCentered,
            sdl2::video::PosCentered,
            800,
            600,
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

    let _ = draw(&renderer, &State { circle_position: (100, 100) });
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
fn draw(renderer: &sdl2::render::Renderer, state: &State) -> sdl2::SdlResult<()> {
    try!(renderer.set_draw_color(sdl2::pixels::RGB(0, 0, 0)));
    try!(renderer.clear());
    try!(drawing::draw_circle(renderer, state.circle_position, 30, sdl2::Pixels::RGB(255, 0, 0)));
    Ok(())
}
