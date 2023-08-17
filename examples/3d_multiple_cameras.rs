//! Demonstrates how to have multiple prank cameras simultaneously.

use bevy::prelude::*;
use bevy_prank::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PrankPlugin::default()))
        .add_systems(Startup, setup)
        .add_systems(Update, camera_switch)
        .run();
}

#[derive(Component)]
struct GameCamera;

#[derive(Component)]
struct FrontView;

#[derive(Component)]
struct TopView;

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
        Name::new("GameCamera"),
        GameCamera,
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 2.0, 0.0),
            ..default()
        },
    ));

    commands.spawn((
        Name::new("FrontView"),
        FrontView,
        Prank3d {
            is_active: false,
            ..default()
        },
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 1.0, 0.0)
                .looking_at(Vec3::new(0.0, 0.5, -8.0), Vec3::Y),
            camera: Camera {
                is_active: false,
                ..default()
            },
            ..default()
        },
    ));

    commands.spawn((
        Name::new("TopView"),
        TopView,
        Prank3d {
            is_active: false,
            ..default()
        },
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 5.0, -5.0)
                .looking_at(Vec3::new(0.0, 0.5, -8.0), Vec3::Y),
            camera: Camera {
                is_active: false,
                ..default()
            },
            ..default()
        },
    ));
}

fn camera_switch(
    mut game_camera: Query<&mut Camera, With<GameCamera>>,
    mut front_view: Query<(&mut Camera, &mut Prank3d), (With<FrontView>, Without<GameCamera>)>,
    mut top_view: Query<
        (&mut Camera, &mut Prank3d),
        (With<TopView>, Without<GameCamera>, Without<FrontView>),
    >,
    keyboard: Res<Input<KeyCode>>,
) {
    let mut gc_camera = game_camera.single_mut();
    let (mut fv_camera, mut fv_prank) = front_view.single_mut();
    let (mut tv_camera, mut tv_prank) = top_view.single_mut();

    if keyboard.just_pressed(KeyCode::Key1) {
        gc_camera.is_active = true;

        fv_camera.is_active = false;
        fv_prank.is_active = false;

        tv_camera.is_active = false;
        tv_prank.is_active = false;
    }
    if keyboard.just_pressed(KeyCode::Key2) {
        gc_camera.is_active = false;

        fv_camera.is_active = true;
        fv_prank.is_active = true;

        tv_camera.is_active = false;
        tv_prank.is_active = false;
    }
    if keyboard.just_pressed(KeyCode::Key3) {
        gc_camera.is_active = false;

        fv_camera.is_active = false;
        fv_prank.is_active = false;

        tv_camera.is_active = true;
        tv_prank.is_active = true;
    }
}
