use allocator_api2::alloc::Allocator;

use crate::{Ctx, ecs::Ecs};

pub mod debug;
pub mod draw;
pub mod navigation;

/// A system that can be called by the ECS
pub type SystemFn<A: Allocator + Clone> =
    for<'gs> fn(ctx: &mut Ctx<'gs, A>, prev: &Ecs<A>, next: &mut Ecs<A>) -> anyhow::Result<()>;
