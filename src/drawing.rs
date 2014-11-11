/*!
  Helpers for drawing shapes to the screen.
*/

extern crate sdl2;

/// Draws a circle with the specified position, radius and colour.
pub fn draw_circle(
        renderer: &sdl2::render::Renderer,
        position: (i32, i32),
        radius: i32,
        color: sdl2::pixels::Color)
        -> sdl2::SdlResult<()> {
    let old_color = try!(renderer.get_draw_color());
    try!(renderer.set_draw_color(color));
    let (x, y) = position;
    let real_radius = (radius as f64) - 0.5;

    // Draw a single centre line first
    let start_pt = sdl2::rect::Point { x: x - (radius - 1), y: y };
    let end_pt = sdl2::rect::Point { x: x + (radius - 1), y: y };
    try!(renderer.draw_line(start_pt, end_pt));

    // Draw successive lines above and below the center
    for i in range(1, radius) {
        let h = real_radius - (i as f64);
        let half_w = ((2.0 * real_radius * h) - (h*h)).sqrt() as i32;

        for &py in [y + i, y - i].iter() {
            let start_pt = sdl2::rect::Point { x: x - half_w, y: py };
            let end_pt = sdl2::rect::Point { x: x + half_w, y: py };
            try!(renderer.draw_line(start_pt, end_pt));
        }
    }

    renderer.set_draw_color(old_color)
}
