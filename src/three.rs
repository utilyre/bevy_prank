use self::input::{Prank3dDirection, Prank3dInputPlugin, Prank3dMode, Prank3dRotation};
use bevy::{
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};
use std::f32::consts;

pub mod input;

pub(super) struct Prank3dPlugin {
    pub(super) default_mode_input: bool,
    pub(super) default_direction_input: bool,
    pub(super) default_rotaion_input: bool,
}

impl Plugin for Prank3dPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Prank3dInputPlugin {
            default_mode_input: self.default_mode_input,
            default_direction_input: self.default_direction_input,
            default_rotaion_input: self.default_rotaion_input,
        });

        app.init_resource::<Prank3dActive>();
        app.register_type::<Prank3d>();

        app.add_systems(PreUpdate, active);
        app.add_systems(Update, cursor);
        app.add_systems(Update, movement);
        app.add_systems(Update, orientation);
    }
}

#[derive(Default, Reflect, Resource)]
#[reflect(Resource)]
pub struct Prank3dActive(pub Option<Entity>);

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

fn active(pranks: Query<(Entity, &Camera), With<Prank3d>>, mut active: ResMut<Prank3dActive>) {
    *active = Prank3dActive(
        pranks
            .iter()
            .find(|(_, camera)| camera.is_active)
            .map(|(entity, _)| entity),
    );
}

fn cursor(mut window: Query<&mut Window, With<PrimaryWindow>>, mode: Res<Prank3dMode>) {
    let mut window = window.single_mut();

    match *mode {
        Prank3dMode::Fly => {
            window.cursor.visible = false;
            window.cursor.grab_mode = CursorGrabMode::Locked;
        }
        Prank3dMode::Offset => {
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
    mut direction: EventReader<Prank3dDirection>,
    active: Res<Prank3dActive>,
    mut pranks: Query<(&mut Transform, &Prank3d)>,
    time: Res<Time>,
) {
    let direction: Vec3 = direction.iter().sum();
    let Some(entity) = active.0 else {
        return;
    };
    let (mut transform, prank) = pranks.get_mut(entity).expect("already checked");

    transform.translation += prank.speed * direction * time.delta_seconds();
}

fn orientation(
    mut rotation: EventReader<Prank3dRotation>,
    active: Res<Prank3dActive>,
    mut pranks: Query<(&mut Transform, &mut Prank3d)>,
    time: Res<Time>,
) {
    let rotation: Vec2 = rotation.iter().sum();
    let Some(entity) = active.0 else {
        return;
    };
    let (mut transform, mut prank) = pranks.get_mut(entity).expect("already checked");

    prank.pitch = (prank.pitch - prank.sensitivity.y * rotation.y * time.delta_seconds())
        .clamp(-consts::FRAC_PI_2, consts::FRAC_PI_2);
    prank.yaw -= prank.sensitivity.x * rotation.x * time.delta_seconds();

    transform.rotation = Quat::from_euler(EulerRot::YXZ, prank.yaw, prank.pitch, 0.0);
}
