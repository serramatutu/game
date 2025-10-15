use anyhow::Result;
use engine::animation::AnimationCursor;
use engine::coords::convert::screen_rect_to_sdl;
use engine::coords::{WorldPoint, WorldRect, WorldSize};
use sdl3::pixels::Color;

use crate::consts::PIXEL_TO_WORLD;
use crate::global_state::Ctx;

const SPEED_S: f32 = 5.0;

#[derive(Clone)]
pub(crate) struct Zorb {
    pub body_anim: AnimationCursor,
    pub face_anim: AnimationCursor,

    pub pos: WorldPoint,
    pub target: WorldPoint,
}

impl Zorb {
    pub fn update_and_render<'gamestatic>(
        &mut self,
        ctx: &'gamestatic mut Ctx<'gamestatic, 'gamestatic>,
    ) -> Result<()> {
        let diff = self.target - self.pos;
        let speed = SPEED_S.min(diff.length());

        if speed > f32::EPSILON {
            let movement = diff.normalize() * speed;
            self.pos += movement;
        }

        ctx.canvas.set_draw_color(Color::RGB(255, 0, 0));

        let sprite = ctx.resources.sprites.get(ctx.resource_ids.zorb_sprite);

        let anims = [
            (sprite.get_animation("body:walk"), &mut self.body_anim),
            (sprite.get_animation("face:cute"), &mut self.face_anim),
        ];

        for (anim, cursor) in anims {
            let layer_cels = anim.update_cursor_loop(cursor, ctx.now_ms);
            for cel_i in layer_cels.iter() {
                let cel = sprite.cels[*cel_i as usize];

                // TODO: proper pixel to world conversion somewhere
                let world_size = WorldSize::new(cel.w * PIXEL_TO_WORLD, cel.h * PIXEL_TO_WORLD);
                let world_rect = WorldRect::new(self.pos, world_size);
                let screen_box = ctx.camera.world_to_screen_rect(&world_rect);
                ctx.canvas.copy(
                    &sprite.tex,
                    Some(cel),
                    Some(screen_rect_to_sdl(&screen_box)),
                )?;
            }
        }

        Ok(())
    }
}
