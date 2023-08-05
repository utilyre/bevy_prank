//! Demonstrates a very basic setup of the plugin.

use bevy::prelude::*;
use bevy_prank::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PrankPlugin::default()))
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
        Name::new("Prank3d"),
        Prank3d::default(),
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 2.0, 0.0),
            ..default()
        },
    ));
}
