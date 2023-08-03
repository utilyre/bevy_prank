use bevy::{input::mouse::MouseMotion, prelude::*};
use std::f32::consts;

pub mod prelude;

pub struct PrankPlugin {
    default_direction_input: bool,
    default_orientation_input: bool,
}

impl Default for PrankPlugin {
    fn default() -> Self {
        Self {
            default_direction_input: true,
            default_orientation_input: true,
        }
    }
}

impl Plugin for PrankPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Prank3dAbsoluteDirection>();
        app.add_event::<Prank3dRelativeDirection>();
        app.add_event::<Prank3dRotation>();
        app.register_type::<Prank3d>();

        if self.default_direction_input {
            app.add_systems(PreUpdate, direction_input);
        }
        if self.default_orientation_input {
            app.add_systems(PreUpdate, rotation_input);
        }

        app.add_systems(Update, movement);
        app.add_systems(Update, orientation);
    }
}

#[derive(Event)]
pub struct Prank3dAbsoluteDirection(pub Vec3);

#[derive(Event)]
pub struct Prank3dRelativeDirection(pub Vec3);

fn direction_input(
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

#[derive(Event)]
pub struct Prank3dRotation(pub Vec2);

fn rotation_input(
    mut rotation_event: EventWriter<Prank3dRotation>,
    mut mouse_event: EventReader<MouseMotion>,
) {
    let rotation = mouse_event.iter().fold(Vec2::ZERO, |acc, x| acc + x.delta);
    rotation_event.send(Prank3dRotation(rotation));
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
