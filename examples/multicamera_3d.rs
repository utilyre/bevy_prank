//! Demonstrates how to have multiple prank cameras simultaneously.

use bevy::prelude::*;
use bevy_prank::prelude::*;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);
    app.add_plugins(PrankPlugin);

    app.add_systems(Startup, setup);
    app.add_systems(Update, camera_switch);

    app.run();
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
        Prank3d::default(),
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
        Prank3d::default(),
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 5.0, -8.0)
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
    mut front_view: Query<&mut Camera, (With<FrontView>, Without<GameCamera>)>,
    mut top_view: Query<&mut Camera, (With<TopView>, Without<GameCamera>, Without<FrontView>)>,
    keyboard: Res<Input<KeyCode>>,
) {
    let mut gc_camera = game_camera.single_mut();
    let mut fv_camera = front_view.single_mut();
    let mut tv_camera = top_view.single_mut();

    // to switch cameras, just set its `is_active` property of `bevy::render::camera::Camera`
    // make sure only one Camera with `Prank3d` component is active at a time
    if keyboard.just_pressed(KeyCode::Key1) {
        gc_camera.is_active = true;
        fv_camera.is_active = false;
        tv_camera.is_active = false;
    }
    if keyboard.just_pressed(KeyCode::Key2) {
        gc_camera.is_active = false;
        fv_camera.is_active = true;
        tv_camera.is_active = false;
    }
    if keyboard.just_pressed(KeyCode::Key3) {
        gc_camera.is_active = false;
        fv_camera.is_active = false;
        tv_camera.is_active = true;
    }
}
