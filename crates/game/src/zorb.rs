use anyhow::Result;
use engine::coords::convert::screen_rect_to_sdl;
use engine::coords::{WorldPoint, WorldRect, WorldSize};
use sdl3::pixels::Color;

use crate::global_state::Ctx;

const SPEED_S: f32 = 5.0;

#[derive(Clone)]
pub(crate) struct Zorb {
    pub pos: WorldPoint,
    pub target: WorldPoint,
}

impl Zorb {
    pub fn update_and_render<'a>(&mut self, ctx: &'a mut Ctx<'a, 'a>) -> Result<()> {
        let diff = self.target - self.pos;
        let speed = SPEED_S.min(diff.length());

        if speed > f32::EPSILON {
            let movement = diff.normalize() * speed;
            self.pos += movement;
        }

        ctx.canvas.set_draw_color(Color::RGB(255, 0, 0));

        let world_rect = WorldRect::new(self.pos, WorldSize::new(50.0, 50.0));
        let screen_box = ctx.camera.world_to_screen_rect(&world_rect);

        let tex = ctx.resources.sprites.get(ctx.resource_ids.zorb_face);
        ctx.canvas
            .copy(&tex.tex, None, Some(screen_rect_to_sdl(&screen_box)))?;

        Ok(())
    }
}
