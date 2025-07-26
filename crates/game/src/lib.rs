use anyhow::Result;
use sdl3::{
    pixels::Color,
    render::{FRect, WindowCanvas},
};

#[unsafe(no_mangle)]
pub fn init() {
    print!("init");
}

#[unsafe(no_mangle)]
pub fn update_and_render(canvas: &mut WindowCanvas) -> Result<()> {
    canvas.set_draw_color(Color::RGB(255, 0, 0));
    canvas.draw_rect(FRect {
        x: 10.0,
        y: 10.0,
        w: 50.0,
        h: 50.0,
    })?;
    Ok(())
}
