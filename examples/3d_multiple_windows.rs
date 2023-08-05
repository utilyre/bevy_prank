//! Demonstrates having a separate window to spectate the world in real-time.

use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, render::camera::RenderTarget,
    window::WindowRef,
};
use bevy_prank::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            FrameTimeDiagnosticsPlugin,
            PrankPlugin::default(),
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(AmbientLight {
        brightness: 0.1,
        ..default()
    });

    commands.spawn((
        Name::new("Ground"),
        PbrBundle {
            transform: Transform::from_xyz(0.0, -0.5, 0.0),
            mesh: meshes.add(shape::Box::new(50.0, 1.0, 50.0).into()),
            material: materials.add(Color::DARK_GREEN.into()),
            ..default()
        },
    ));

    commands.spawn((
        Name::new("Cube"),
        PbrBundle {
            transform: Transform::from_xyz(0.0, 0.5, -8.0),
            mesh: meshes.add(shape::Cube::new(1.0).into()),
            material: materials.add(Color::WHITE.into()),
            ..default()
        },
    ));

    commands.spawn((
        Name::new("Camera3d"),
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 2.0, 0.0),
            ..default()
        },
    ));

    let debug_window = commands
        .spawn((
            Name::new("DebugWindow"),
            Window {
                title: "Debug Window".into(),
                ..default()
            },
        ))
        .id();

    commands.spawn((
        Name::new("Prank3d"),
        Prank3d::default(),
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 5.0, 0.0)
                .looking_at(Vec3::new(0.0, 0.5, -8.0), Vec3::Y),
            camera: Camera {
                target: RenderTarget::Window(WindowRef::Entity(debug_window)),
                ..default()
            },
            ..default()
        },
    ));
}
