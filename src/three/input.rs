use super::{sync_active, Prank3d, Prank3dActive};
use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
};

pub(super) struct Prank3dInputPlugin;

impl Plugin for Prank3dInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<Prank3dMode>()
            .add_event::<Prank3dMovement>()
            .add_event::<Prank3dRotation>()
            .add_systems(
                PreUpdate,
                (
                    mode,
                    speed_factor.run_if(in_state(Prank3dMode::Fly)),
                    movement.run_if(not(in_state(Prank3dMode::None))),
                    rotation.run_if(in_state(Prank3dMode::Fly)),
                )
                    .after(sync_active),
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

#[derive(Event)]
pub(super) struct Prank3dMovement(pub(super) Vec3);

#[derive(Event)]
pub(super) struct Prank3dRotation(pub(super) Vec2);

fn mode(
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
            if !mouse.pressed(MouseButton::Right) {
                mode.set(Prank3dMode::None);
            }
        }
        Prank3dMode::Offset => {
            if !mouse.pressed(MouseButton::Middle) {
                mode.set(Prank3dMode::None);
            }
        }
        Prank3dMode::None => {
            if mouse.pressed(MouseButton::Right) {
                mode.set(Prank3dMode::Fly);
            } else if mouse.pressed(MouseButton::Middle) {
                mode.set(Prank3dMode::Offset);
            }
        }
    }
}

fn speed_factor(
    active: Res<Prank3dActive>,
    mut pranks: Query<&mut Prank3d>,
    mut wheel: EventReader<MouseWheel>,
) {
    let Some(entity) = active.0 else {
        return;
    };
    let mut prank = pranks.get_mut(entity).expect("already checked");

    prank.speed_factor =
        (prank.speed_factor + 0.1 * wheel.iter().fold(0.0, |acc, x| acc + x.y)).clamp(0.1, 10.0);
}

fn movement(
    mode: Res<State<Prank3dMode>>,
    active: Res<Prank3dActive>,
    pranks: Query<(&GlobalTransform, &Prank3d)>,
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

            prank.speed_factor * movement.normalize_or_zero()
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

fn rotation(
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
