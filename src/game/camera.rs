use bevy::{
    ecs::{system::RunSystemOnce as _, world::Command},
    prelude::*,
};

/// Plugin handling the camera. Takes in [`Transform`] and [`Camera`] components
/// registers the [`MainCamera`] and [`MainCameraTarget`] components, and provides
/// a [`SpawnMainCamera`] command.
pub(super) fn plugin(app: &mut App) {
    app.register_type::<MainCamera>()
        .register_type::<MainCameraTarget>();
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Default, Reflect)]
#[reflect(Component)]
pub struct MainCamera;

#[derive(Component, Debug, Clone, Copy, PartialEq, Default, Reflect)]
#[reflect(Component)]
pub struct MainCameraTarget;

// A command to spawn the main camera.
#[derive(Debug)]
pub struct SpawnMainCamera {
    pub transform: Transform,
}

impl Command for SpawnMainCamera {
    fn apply(self, world: &mut World) {
        world.run_system_once_with(self, spawn_main_camera);
    }
}

fn spawn_main_camera(In(config): In<SpawnMainCamera>, mut commands: Commands) {
    commands.spawn((
        Name::new("MainCamera"),
        MainCamera,
        Camera3dBundle {
            transform: config.transform,
            camera: Camera {
                order: -1,
                clear_color: ClearColorConfig::Custom(Color::srgba(0.0, 0.0, 0.0, 0.0)),
                ..default()
            },
            ..default()
        },
    ));
}
