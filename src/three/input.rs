use super::{Prank3d, Prank3dActive};
use bevy::{input::mouse::MouseMotion, prelude::*};
use std::iter::Sum;

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

#[derive(Default, Reflect, Resource)]
#[reflect(Resource)]
pub enum Prank3dMode {
    Fly,
    #[default]
    None,
}

#[derive(Event)]
pub struct Prank3dDirection(pub Vec3);

impl<'a> Sum<&'a Prank3dDirection> for Vec3 {
    fn sum<I: Iterator<Item = &'a Prank3dDirection>>(iter: I) -> Self {
        iter.fold(Vec3::ZERO, |acc, direction| acc + direction.0)
    }
}

#[derive(Event)]
pub struct Prank3dRotation(pub Vec2);

impl<'a> Sum<&'a Prank3dRotation> for Vec2 {
    fn sum<I: Iterator<Item = &'a Prank3dRotation>>(iter: I) -> Self {
        iter.fold(Vec2::ZERO, |acc, direction| acc + direction.0)
    }
}

fn mode_input(
    active: Res<Prank3dActive>,
    mut mode: ResMut<Prank3dMode>,
    mouse: Res<Input<MouseButton>>,
) {
    if active.0.is_none() {
        *mode = Prank3dMode::None;
        return;
    }

    if mouse.just_released(MouseButton::Right) {
        *mode = Prank3dMode::None;
    }
    if mouse.just_pressed(MouseButton::Right) {
        *mode = Prank3dMode::Fly;
    }
}

fn direction_input(
    mode: Res<Prank3dMode>,
    active: Res<Prank3dActive>,
    pranks: Query<(&GlobalTransform, &Camera), With<Prank3d>>,
    mut direction: EventWriter<Prank3dDirection>,
    keyboard: Res<Input<KeyCode>>,
) {
    let Some(entity) = active.0 else {
        return;
    };
    let (transform, _) = pranks.get(entity).expect("already checked");

    direction.send(Prank3dDirection(match *mode {
        Prank3dMode::Fly => {
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

            direction
        }
        Prank3dMode::None => Vec3::ZERO,
    }));
}

fn rotation_input(
    mode: Res<Prank3dMode>,
    mut rotation: EventWriter<Prank3dRotation>,
    mut mouse: EventReader<MouseMotion>,
) {
    rotation.send(Prank3dRotation(match *mode {
        Prank3dMode::Fly => mouse
            .iter()
            .fold(Vec2::ZERO, |acc, motion| acc + motion.delta),
        Prank3dMode::None => Vec2::ZERO,
    }));
}
