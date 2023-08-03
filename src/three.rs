use self::events::{direction_input, rotation_input, Prank3dDirection, Prank3dRotation};
use bevy::{
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};
use std::f32::consts;

pub mod events;

pub(super) struct Prank3dPlugin {
    pub(super) default_mode_management: bool,
    pub(super) default_direction_input: bool,
    pub(super) default_rotaion_input: bool,
}

impl Plugin for Prank3dPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Prank3d>();
        app.add_event::<Prank3dDirection>();
        app.add_event::<Prank3dRotation>();

        if self.default_mode_management {
            app.add_systems(PreUpdate, mode_management);
        }
        if self.default_direction_input {
            app.add_systems(PreUpdate, direction_input);
        }
        if self.default_rotaion_input {
            app.add_systems(PreUpdate, rotation_input);
        }

        app.add_systems(Update, movement);
        app.add_systems(Update, orientation);
    }
}

#[derive(Reflect)]
pub enum Prank3dMode {
    Fly,
    None,
}

impl Prank3dMode {
    pub fn is_fly(&self) -> bool {
        if let Self::Fly = self {
            true
        } else {
            false
        }
    }

    pub fn is_none(&self) -> bool {
        if let Self::None = self {
            true
        } else {
            false
        }
    }
}

#[derive(Reflect, Component)]
#[reflect(Component)]
pub struct Prank3d {
    pub mode: Prank3dMode,
    pub speed: f32,
    pub sensitivity: Vec2,
    pub pitch: f32,
    pub yaw: f32,
}

impl Default for Prank3d {
    fn default() -> Self {
        Self {
            mode: Prank3dMode::None,
            speed: 5.0,
            sensitivity: Vec2::splat(0.08),
            pitch: 0.0,
            yaw: 0.0,
        }
    }
}

fn mode_management(
    mut pranks: Query<(&Camera, &mut Prank3d)>,
    mouse: Res<Input<MouseButton>>,
    mut window: Query<&mut Window, With<PrimaryWindow>>,
) {
    let mut window = window.single_mut();

    for (camera, mut prank) in pranks.iter_mut() {
        if camera.is_active {
            if mouse.just_pressed(MouseButton::Right) {
                window.cursor.visible = false;
                window.cursor.grab_mode = CursorGrabMode::Locked;
                prank.mode = Prank3dMode::Fly;
            }
        }

        if mouse.just_released(MouseButton::Right) {
            window.cursor.visible = true;
            window.cursor.grab_mode = CursorGrabMode::None;
            prank.mode = Prank3dMode::None;
        }
    }
}

fn movement(
    mut pranks: Query<(&mut Transform, &Camera, &Prank3d), With<Prank3d>>,
    mut direction: EventReader<Prank3dDirection>,
    time: Res<Time>,
) {
    let Some((mut transform, _, prank)) = pranks
        .iter_mut()
        .find(|(_, camera, prank)| camera.is_active && prank.mode.is_fly())
    else {
        return;
    };
    let direction = direction.iter().fold(Vec3::ZERO, |acc, x| acc + x.0);

    transform.translation += prank.speed * direction.normalize_or_zero() * time.delta_seconds();
}

fn orientation(
    mut pranks: Query<(&mut Transform, &Camera, &mut Prank3d)>,
    mut rotation: EventReader<Prank3dRotation>,
    time: Res<Time>,
) {
    let Some((mut transform, _, mut prank)) = pranks
        .iter_mut()
        .find(|(_, camera, prank)| camera.is_active && prank.mode.is_fly())
    else {
        return;
    };
    let rotation = rotation.iter().fold(Vec2::ZERO, |acc, x| acc + x.0);

    prank.pitch = (prank.pitch - prank.sensitivity.y * rotation.y * time.delta_seconds())
        .clamp(-consts::FRAC_PI_2, consts::FRAC_PI_2);
    prank.yaw -= prank.sensitivity.x * rotation.x * time.delta_seconds();

    transform.rotation = Quat::from_euler(EulerRot::YXZ, prank.yaw, prank.pitch, 0.0);
}
