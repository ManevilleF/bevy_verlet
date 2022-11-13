use bevy::ecs::prelude::Resource;

#[derive(Debug, Clone, Resource)]
pub enum VerletTimeStep {
    DeltaTime,
    FixedDeltaTime(f64),
}
