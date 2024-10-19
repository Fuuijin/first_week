use bevy::{
    ecs::{system::RunSystemOnce as _, world::Command},
    prelude::*,
};

use super::camera::SpawnMainCamera;

pub const GRID_SIZE: (f32, f32) = (128.0, 128.0);

/// Plugin for setting up the scene
pub(super) fn plugin(app: &mut App) {
    app.register_type::<Scene>();
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Default, Reflect)]
#[reflect(Component)]
pub struct Ground;

#[derive(Component, Debug, Clone, Copy, PartialEq, Default, Reflect)]
#[reflect(Component)]
pub struct Scene;

// A command to spawn the main camera.
#[derive(Debug)]
pub struct SetupScene {
    pub grid_size: (f32, f32),
}

impl Command for SetupScene {
    fn apply(self, world: &mut World) {
        world.run_system_once_with(self, setup_scene);
        SpawnMainCamera {
            transform: Transform::from_xyz(GRID_SIZE.0, 95.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        }
        .apply(world);
    }
}

fn setup_scene(
    In(config): In<SetupScene>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_translation(Vec3::ONE).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
    commands.spawn((
        Name::new("Ground"),
        Ground,
        PbrBundle {
            mesh: meshes.add(
                Plane3d::default()
                    .mesh()
                    .size(config.grid_size.0, config.grid_size.0),
            ),
            material: materials.add(Color::srgb(0.3, 0.5, 0.3)),
            ..default()
        },
    ));
}
