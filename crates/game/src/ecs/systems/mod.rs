use crate::{Ctx, ecs::Ecs};

pub mod debug;
pub mod navigation;

/// A system that can be called by the ECS
pub type SystemFn =
    for<'gs> fn(ctx: &mut Ctx<'gs, 'gs>, prev: &Ecs, next: &mut Ecs) -> anyhow::Result<()>;
