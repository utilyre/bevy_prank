use bevy::{input::mouse::MouseMotion, prelude::*};

#[derive(Event)]
pub struct Prank3dAbsoluteDirection(pub Vec3);

#[derive(Event)]
pub struct Prank3dRelativeDirection(pub Vec3);

#[derive(Event)]
pub struct Prank3dRotation(pub Vec2);

pub(super) fn direction_input(
    mut adir_event: EventWriter<Prank3dAbsoluteDirection>,
    mut rdir_event: EventWriter<Prank3dRelativeDirection>,
    keyboard: Res<Input<KeyCode>>,
) {
    let mut adir = Vec3::ZERO;
    let mut rdir = Vec3::ZERO;

    if keyboard.pressed(KeyCode::ShiftLeft) {
        if keyboard.pressed(KeyCode::W) {
            adir += Vec3::NEG_Z;
        }
        if keyboard.pressed(KeyCode::A) {
            adir += Vec3::NEG_X;
        }
        if keyboard.pressed(KeyCode::S) {
            adir += Vec3::Z;
        }
        if keyboard.pressed(KeyCode::D) {
            adir += Vec3::X;
        }
    } else {
        if keyboard.pressed(KeyCode::W) {
            rdir += Vec3::NEG_Z;
        }
        if keyboard.pressed(KeyCode::A) {
            rdir += Vec3::NEG_X;
        }
        if keyboard.pressed(KeyCode::S) {
            rdir += Vec3::Z;
        }
        if keyboard.pressed(KeyCode::D) {
            rdir += Vec3::X;
        }
    }

    if keyboard.pressed(KeyCode::E) {
        adir += Vec3::Y;
    }
    if keyboard.pressed(KeyCode::Q) {
        adir += Vec3::NEG_Y;
    }

    adir_event.send(Prank3dAbsoluteDirection(adir));
    rdir_event.send(Prank3dRelativeDirection(rdir));
}

pub(super) fn rotation_input(
    mut rotation_event: EventWriter<Prank3dRotation>,
    mut mouse_event: EventReader<MouseMotion>,
) {
    let rotation = mouse_event.iter().fold(Vec2::ZERO, |acc, x| acc + x.delta);
    rotation_event.send(Prank3dRotation(rotation));
}
