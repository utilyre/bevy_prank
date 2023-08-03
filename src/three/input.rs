use super::Prank3d;
use bevy::{input::mouse::MouseMotion, prelude::*};

pub(super) struct Prank3dInputPlugin {
    pub(super) default_mode_input: bool,
    pub(super) default_direction_input: bool,
    pub(super) default_rotaion_input: bool,
}

impl Plugin for Prank3dInputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Prank3dMode>();
        app.add_event::<Prank3dDirection>();
        app.add_event::<Prank3dRotation>();

        if self.default_mode_input {
            app.add_systems(PreUpdate, mode_input);
        }
        if self.default_direction_input {
            app.add_systems(PreUpdate, direction_input);
        }
        if self.default_rotaion_input {
            app.add_systems(PreUpdate, rotation_input);
        }
    }
}

#[derive(Default, Resource)]
pub enum Prank3dMode {
    Fly,
    #[default]
    None,
}

#[derive(Event)]
pub struct Prank3dDirection(pub Vec3);

#[derive(Event)]
pub struct Prank3dRotation(pub Vec2);

fn mode_input(
    pranks: Query<&Camera, With<Prank3d>>,
    mut mode: ResMut<Prank3dMode>,
    mouse: Res<Input<MouseButton>>,
) {
    if !pranks.iter().any(|camera| camera.is_active) {
        *mode = Prank3dMode::None;
    }

    if mouse.just_released(MouseButton::Right) {
        *mode = Prank3dMode::None;
    }
    if mouse.just_pressed(MouseButton::Right) {
        *mode = Prank3dMode::Fly;
    }
}

fn direction_input(
    pranks: Query<(&GlobalTransform, &Camera), With<Prank3d>>,
    mut direction_event: EventWriter<Prank3dDirection>,
    keyboard: Res<Input<KeyCode>>,
) {
    let Some((transform, _)) = pranks
        .iter()
        .find(|(_, camera)| camera.is_active)
    else {
        return;
    };
    let mut direction = Vec3::ZERO;

    if keyboard.pressed(KeyCode::W) {
        direction += transform.forward();
    }
    if keyboard.pressed(KeyCode::A) {
        direction += transform.left();
    }
    if keyboard.pressed(KeyCode::S) {
        direction += transform.back();
    }
    if keyboard.pressed(KeyCode::D) {
        direction += transform.right();
    }
    if keyboard.pressed(KeyCode::ShiftLeft) {
        direction = Vec3::new(direction.x, 0.0, direction.z);
    }
    if keyboard.pressed(KeyCode::E) {
        direction += Vec3::Y;
    }
    if keyboard.pressed(KeyCode::Q) {
        direction += Vec3::NEG_Y;
    }

    direction_event.send(Prank3dDirection(direction));
}

fn rotation_input(
    mut rotation_event: EventWriter<Prank3dRotation>,
    mut mouse_event: EventReader<MouseMotion>,
) {
    let rotation = mouse_event.iter().fold(Vec2::ZERO, |acc, x| acc + x.delta);
    rotation_event.send(Prank3dRotation(rotation));
}
