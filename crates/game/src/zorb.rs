use anyhow::Result;
use engine::animation::Animation;
use engine::coords::convert::screen_rect_to_sdl;
use engine::coords::{WorldPoint, WorldRect};
use sdl3::pixels::Color;

use crate::consts::PIXEL_TO_WORLD;
use crate::global_state::Ctx;

const SPEED_S: f32 = 5.0;

#[derive(Clone)]
pub(crate) struct Zorb {
    pub anim: Animation<usize>,
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

        let frame = match self.anim.update(ctx.now_ms) {
            Some(f) => *f,
            None => *self.anim.start(ctx.now_ms),
        };

        let tex = ctx.resources.sprites.get(ctx.resource_ids.zorb_body);
        let world_size = tex.get_world_size(PIXEL_TO_WORLD, frame).expect("TODO");
        let world_rect = WorldRect::new(self.pos, world_size);
        let screen_box = ctx.camera.world_to_screen_rect(&world_rect);

        ctx.canvas.copy(
            &tex.tex,
            tex.get_frame_rect(frame),
            Some(screen_rect_to_sdl(&screen_box)),
        )?;

        Ok(())
    }
}
