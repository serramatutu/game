use anyhow::Result;
use engine::coords::WorldPoint;
use engine::hooks::UpdateAndRenderParams;
use sdl3::pixels::Color;

const SPEED_S: f32 = 5.0;

pub(crate) struct Zorb {
    pub pos: WorldPoint,
    pub target: WorldPoint,
}

impl Zorb {
    pub fn update_and_render(&mut self, params: &mut UpdateAndRenderParams) -> Result<()> {
        let diff = self.target - self.pos;
        let speed = SPEED_S.min(diff.length());

        if speed > f32::EPSILON {
            let movement = diff.normalize() * speed;
            self.pos += movement;
        }

        // TODO: world space to screen space
        params.canvas.set_draw_color(Color::RGB(255, 0, 0));
        params.canvas.draw_rect(sdl3::render::FRect {
            x: self.pos.x,
            y: self.pos.y,
            w: 50.0,
            h: 50.0,
        })?;

        Ok(())
    }
}
