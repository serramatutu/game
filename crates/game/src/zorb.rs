use anyhow::Result;
use engine::coords::convert::screen_rect_to_sdl;
use engine::coords::{WorldPoint, WorldRect, WorldSize};
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

        params.canvas.set_draw_color(Color::RGB(255, 0, 0));

        let world_rect = WorldRect::new(self.pos, WorldSize::new(50.0, 50.0));
        let screen_box = params.camera.world_to_screen_rect(&world_rect);
        params.canvas.fill_rect(screen_rect_to_sdl(&screen_box))?;

        Ok(())
    }
}
