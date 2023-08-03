use self::events::{
    direction_input, mode_input, rotation_input, Prank3dDirection, Prank3dMode, Prank3dRotation,
};
use bevy::{
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};
use std::f32::consts;

pub mod events;

pub(super) struct Prank3dPlugin {
    pub(super) default_mode_input: bool,
    pub(super) default_direction_input: bool,
    pub(super) default_rotaion_input: bool,
}

impl Plugin for Prank3dPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Prank3d>();
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

        app.add_systems(Update, cursor);
        app.add_systems(Update, movement);
        app.add_systems(Update, orientation);
    }
}

#[derive(Reflect, Component)]
#[reflect(Component)]
pub struct Prank3d {
    pub speed: f32,
    pub sensitivity: Vec2,
    pub pitch: f32,
    pub yaw: f32,
}

impl Default for Prank3d {
    fn default() -> Self {
        Self {
            speed: 5.0,
            sensitivity: Vec2::splat(0.08),
            pitch: 0.0,
            yaw: 0.0,
        }
    }
}

fn cursor(mut window: Query<&mut Window, With<PrimaryWindow>>, mode: Res<Prank3dMode>) {
    let mut window = window.single_mut();

    match *mode {
        Prank3dMode::Fly => {
            window.cursor.visible = false;
            window.cursor.grab_mode = CursorGrabMode::Locked;
        }
        Prank3dMode::None => {
            window.cursor.visible = true;
            window.cursor.grab_mode = CursorGrabMode::None;
        }
    }
}

fn movement(
    mode: Res<Prank3dMode>,
    mut direction: EventReader<Prank3dDirection>,
    mut pranks: Query<(&mut Transform, &Camera, &Prank3d)>,
    time: Res<Time>,
) {
    if !matches!(*mode, Prank3dMode::Fly) {
        return;
    }

    let direction = direction.iter().fold(Vec3::ZERO, |acc, x| acc + x.0);
    let Some((mut transform, _, prank)) = pranks
        .iter_mut()
        .find(|(_, camera, _)| camera.is_active)
    else {
        return;
    };

    transform.translation += prank.speed * direction.normalize_or_zero() * time.delta_seconds();
}

fn orientation(
    mode: Res<Prank3dMode>,
    mut rotation: EventReader<Prank3dRotation>,
    mut pranks: Query<(&mut Transform, &Camera, &mut Prank3d)>,
    time: Res<Time>,
) {
    if !matches!(*mode, Prank3dMode::Fly) {
        return;
    }

    let rotation = rotation.iter().fold(Vec2::ZERO, |acc, x| acc + x.0);
    let Some((mut transform, _, mut prank)) = pranks
        .iter_mut()
        .find(|(_, camera, _)| camera.is_active)
    else {
        return;
    };

    prank.pitch = (prank.pitch - prank.sensitivity.y * rotation.y * time.delta_seconds())
        .clamp(-consts::FRAC_PI_2, consts::FRAC_PI_2);
    prank.yaw -= prank.sensitivity.x * rotation.x * time.delta_seconds();

    transform.rotation = Quat::from_euler(EulerRot::YXZ, prank.yaw, prank.pitch, 0.0);
}
