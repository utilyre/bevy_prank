use super::{active, Prank3d, Prank3dActive};
use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
};
use std::iter::Sum;

pub(super) struct Prank3dInputPlugin;

impl Plugin for Prank3dInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<Prank3dMode>();
        app.init_resource::<Prank3dSpeedFactor>();
        app.add_event::<Prank3dMovement>();
        app.add_event::<Prank3dRotation>();

        app.add_systems(
            PreUpdate,
            (
                mode_input,
                speed_factor_input.run_if(in_state(Prank3dMode::Fly)),
                movement_input,
                rotation_input,
            )
                .after(active),
        );
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, States)]
pub(super) enum Prank3dMode {
    Fly,
    Offset,
    #[default]
    None,
}

#[derive(Resource)]
struct Prank3dSpeedFactor(f32);

impl Default for Prank3dSpeedFactor {
    fn default() -> Self {
        Self(1.0)
    }
}

#[derive(Event)]
pub(super) struct Prank3dMovement(Vec3);

impl<'a> Sum<&'a Prank3dMovement> for Vec3 {
    fn sum<I: Iterator<Item = &'a Prank3dMovement>>(iter: I) -> Self {
        iter.fold(Vec3::ZERO, |acc, x| acc + x.0)
    }
}

#[derive(Event)]
pub(super) struct Prank3dRotation(Vec2);

impl<'a> Sum<&'a Prank3dRotation> for Vec2 {
    fn sum<I: Iterator<Item = &'a Prank3dRotation>>(iter: I) -> Self {
        iter.fold(Vec2::ZERO, |acc, x| acc + x.0)
    }
}

fn mode_input(
    active: Res<Prank3dActive>,
    prev_mode: Res<State<Prank3dMode>>,
    mut mode: ResMut<NextState<Prank3dMode>>,
    mouse: Res<Input<MouseButton>>,
) {
    if active.0.is_none() {
        mode.set(Prank3dMode::None);
        return;
    }

    match **prev_mode {
        Prank3dMode::Fly => {
            if mouse.just_released(MouseButton::Right) {
                mode.set(Prank3dMode::None);
            }
        }
        Prank3dMode::Offset => {
            if mouse.just_released(MouseButton::Middle) {
                mode.set(Prank3dMode::None);
            }
        }
        Prank3dMode::None => {
            if mouse.just_pressed(MouseButton::Right) {
                mode.set(Prank3dMode::Fly);
            } else if mouse.just_pressed(MouseButton::Middle) {
                mode.set(Prank3dMode::Offset);
            }
        }
    }
}

fn speed_factor_input(
    mut speed_factor: ResMut<Prank3dSpeedFactor>,
    mut wheel: EventReader<MouseWheel>,
) {
    speed_factor.0 =
        (speed_factor.0 + 0.1 * wheel.iter().fold(0.0, |acc, x| acc + x.y)).clamp(0.1, 10.0);
}

fn movement_input(
    mode: Res<State<Prank3dMode>>,
    active: Res<Prank3dActive>,
    pranks: Query<(&GlobalTransform, &Prank3d), With<Prank3d>>,
    speed_factor: Res<Prank3dSpeedFactor>,
    mut movement: EventWriter<Prank3dMovement>,
    mut motion: EventReader<MouseMotion>,
    keyboard: Res<Input<KeyCode>>,
) {
    let Some(entity) = active.0 else {
        return;
    };
    let (transform, prank) = pranks.get(entity).expect("already checked");

    movement.send(Prank3dMovement(match **mode {
        Prank3dMode::Fly => {
            let mut movement = Vec3::ZERO;

            if keyboard.pressed(KeyCode::W) {
                movement += transform.forward();
            }
            if keyboard.pressed(KeyCode::A) {
                movement += transform.left();
            }
            if keyboard.pressed(KeyCode::S) {
                movement += transform.back();
            }
            if keyboard.pressed(KeyCode::D) {
                movement += transform.right();
            }
            if keyboard.pressed(KeyCode::ShiftLeft) {
                movement = Vec3::new(movement.x, 0.0, movement.z);
            }
            if keyboard.pressed(KeyCode::E) {
                movement += Vec3::Y;
            }
            if keyboard.pressed(KeyCode::Q) {
                movement += Vec3::NEG_Y;
            }

            speed_factor.0 * movement.normalize_or_zero()
        }
        Prank3dMode::Offset => {
            let motion = motion
                .iter()
                .fold(Vec2::ZERO, |acc, motion| acc + motion.delta);

            transform.compute_transform().rotation
                * Vec3::new(
                    prank.sensitivity.x * motion.x,
                    -prank.sensitivity.y * motion.y,
                    0.0,
                )
        }
        Prank3dMode::None => Vec3::ZERO,
    }));
}

fn rotation_input(
    mode: Res<State<Prank3dMode>>,
    mut rotation: EventWriter<Prank3dRotation>,
    mut motion: EventReader<MouseMotion>,
) {
    rotation.send(Prank3dRotation(match **mode {
        Prank3dMode::Fly => motion.iter().fold(Vec2::ZERO, |acc, x| acc + x.delta),
        Prank3dMode::Offset => Vec2::ZERO,
        Prank3dMode::None => Vec2::ZERO,
    }));
}
