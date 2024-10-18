use bevy::{
    ecs::{system::RunSystemOnce as _, world::Command},
    prelude::*,
};

use crate::{screens::Screen, theme::prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Ground>();
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Default, Reflect)]
#[reflect(Component)]
pub struct Ground {
    pub size: (f32, f32),
}

#[derive(Debug)]
pub struct StartSimulation {
    pub step_count: f32,
}

impl Command for StartSimulation {
    fn apply(self, world: &mut World) {
        world.run_system_once_with(self, start_simulation);
    }
}

pub fn draw_ui(mut commands: Commands, ground: Query<&Transform, With<Ground>>) {
    // Draw UI
    commands
        .ui_root()
        .insert(StateScoped(Screen::Gameplay))
        .with_children(|children| {
            children.label(format!("Grid Size: {:?}", ground));
        });
    commands.ui_root().despawn();
}

fn start_simulation(
    In(_config): In<StartSimulation>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
        material: materials.add(Color::srgb_u8(124, 144, 255)),
        transform: Transform::from_xyz(62.0, 0.5, 0.0),
        ..default()
    });
    // Ground
    commands.spawn((
        Ground {
            size: (128.0, 128.0),
        },
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(128.0, 128.0)),
            material: materials.add(Color::srgb(0.3, 0.5, 0.3)),
            ..default()
        },
    ));

    // light
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_translation(Vec3::ONE).looking_at(Vec3::ZERO, Vec3::Y),
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });
}
