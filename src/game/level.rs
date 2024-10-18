use bevy::{ecs::world::Command, prelude::*};

use super::{camera::SpawnMainCamera, scene::SetupScene};

pub(super) fn plugin(_app: &mut App) {
    // No setup required for this plugin.
    // It's still good to have a function here so that we can add some setup
    // later if needed.
}

/// A [`Command`] to spawn the level.
/// Functions that accept only `&mut World` as their parameter implement [`Command`].
/// We use this style when a command requires no configuration.
pub fn spawn_level(world: &mut World) {
    SetupScene {
        grid_size: (128.0, 128.0),
    }
    .apply(world);
    SpawnMainCamera {
        transform: Transform::from_xyz(128.0, 95.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
    }
    .apply(world);
}
