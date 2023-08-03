use self::events::{
    direction_input, rotation_input, Prank3dAbsoluteDirection, Prank3dRelativeDirection,
    Prank3dRotation,
};
use bevy::prelude::*;
use std::f32::consts;

pub mod events;

pub(super) struct Prank3dPlugin {
    pub(super) default_direction_input: bool,
    pub(super) default_rotaion_input: bool,
}

impl Plugin for Prank3dPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Prank3d>();
        app.add_event::<Prank3dAbsoluteDirection>();
        app.add_event::<Prank3dRelativeDirection>();
        app.add_event::<Prank3dRotation>();

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

fn movement(
    mut pranks: Query<(&mut Transform, &Camera, &Prank3d), With<Prank3d>>,
    mut adir: EventReader<Prank3dAbsoluteDirection>,
    mut rdir: EventReader<Prank3dRelativeDirection>,
    time: Res<Time>,
) {
    let adir = adir.iter().fold(Vec3::ZERO, |acc, x| acc + x.0);
    let rdir = rdir.iter().fold(Vec3::ZERO, |acc, x| acc + x.0);

    for (mut transform, camera, prank) in pranks.iter_mut() {
        if !camera.is_active {
            continue;
        }

        let rdir = transform.rotation * rdir;
        transform.translation +=
            prank.speed * (adir + rdir).normalize_or_zero() * time.delta_seconds();
    }
}

fn orientation(
    mut pranks: Query<(&mut Transform, &mut Prank3d)>,
    mut rotation: EventReader<Prank3dRotation>,
    time: Res<Time>,
) {
    let rotation = rotation.iter().fold(Vec2::ZERO, |acc, x| acc + x.0);

    for (mut transform, mut prank) in pranks.iter_mut() {
        prank.pitch = (prank.pitch - prank.sensitivity.y * rotation.y * time.delta_seconds())
            .clamp(-consts::FRAC_PI_2, consts::FRAC_PI_2);
        prank.yaw -= prank.sensitivity.x * rotation.x * time.delta_seconds();

        transform.rotation = Quat::from_euler(EulerRot::YXZ, prank.yaw, prank.pitch, 0.0);
    }
}
