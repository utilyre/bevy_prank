use self::{
    hud::Prank3dHudPlugin,
    input::{Prank3dInputPlugin, Prank3dMode, Prank3dMovement, Prank3dRotation},
};
use crate::Prank3dHudConfig;
use bevy::{
    prelude::*,
    window::{Cursor, CursorGrabMode, PrimaryWindow},
};
use std::f32::consts;

mod hud;
mod input;

pub(super) struct Prank3dPlugin {
    pub(super) hud: Option<Prank3dHudConfig>,
}

impl Plugin for Prank3dPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Prank3dInputPlugin);

        if let Some(config) = self.hud.clone() {
            app.insert_resource(config);
            app.add_plugins(Prank3dHudPlugin);
        }

        app.init_resource::<Prank3dActive>();
        app.register_type::<Prank3d>();

        app.add_systems(PreUpdate, sync_active);
        app.add_systems(
            Update,
            (
                initialize,
                sync_cursor.run_if(
                    |active: Res<Prank3dActive>, mode: Res<State<Prank3dMode>>| {
                        active.is_changed() || mode.is_changed()
                    },
                ),
                movement,
                rotation.run_if(in_state(Prank3dMode::Fly)),
            ),
        );
    }
}

#[derive(Default, Resource)]
struct Prank3dActive(Option<Entity>);

#[derive(Reflect, Component)]
#[reflect(Component)]
pub struct Prank3d {
    pub speed: f32,
    pub speed_factor: f32,
    pub interp_rate: f32,
    pub sensitivity: Vec2,

    pub position: Vec3,
    pub pitch: f32,
    pub yaw: f32,
}

impl Default for Prank3d {
    fn default() -> Self {
        Self {
            speed: 10.0,
            speed_factor: 1.0,
            interp_rate: 0.01,
            sensitivity: Vec2::splat(0.08),

            position: Vec3::ZERO,
            pitch: 0.0,
            yaw: 0.0,
        }
    }
}

fn sync_active(pranks: Query<(Entity, &Camera), With<Prank3d>>, mut active: ResMut<Prank3dActive>) {
    let new = pranks
        .iter()
        .find(|(_, camera)| camera.is_active)
        .map(|(entity, _)| entity);
    if new == active.0 {
        return;
    }

    *active = Prank3dActive(new);
}

fn initialize(mut pranks: Query<(&mut Prank3d, &GlobalTransform), Added<Prank3d>>) {
    for (mut prank, transform) in pranks.iter_mut() {
        let (yaw, pitch, _) = transform
            .compute_transform()
            .rotation
            .to_euler(EulerRot::YXZ);

        prank.position = transform.translation();
        prank.pitch = pitch;
        prank.yaw = yaw;
    }
}

fn sync_cursor(
    mut window: Query<&mut Window, With<PrimaryWindow>>,
    active: Res<Prank3dActive>,
    mode: Res<State<Prank3dMode>>,
    mut initialized: Local<bool>,
    mut cursor: Local<Cursor>,
) {
    let mut window = window.single_mut();

    if !*initialized {
        *initialized = true;
        *cursor = window.cursor;
    }
    if active.0.is_none() {
        *initialized = false;
        window.cursor = *cursor;
        return;
    }

    match **mode {
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
    mut movement: EventReader<Prank3dMovement>,
    active: Res<Prank3dActive>,
    mut pranks: Query<(&mut Transform, &mut Prank3d)>,
    time: Res<Time>,
) {
    let movement = movement.iter().fold(Vec3::ZERO, |acc, x| acc + x.0);
    let Some(entity) = active.0 else {
        return;
    };
    let (mut transform, mut prank) = pranks.get_mut(entity).expect("already checked");

    let s = prank.speed;
    prank.position += s * movement * time.delta_seconds();

    transform.translation = transform.translation.lerp(
        prank.position,
        1.0 - prank.interp_rate.powf(time.delta_seconds()),
    );
}

fn rotation(
    mut rotation: EventReader<Prank3dRotation>,
    active: Res<Prank3dActive>,
    mut pranks: Query<(&mut Transform, &mut Prank3d)>,
    time: Res<Time>,
) {
    let rotation = rotation.iter().fold(Vec2::ZERO, |acc, x| acc + x.0);
    let Some(entity) = active.0 else {
        return;
    };
    let (mut transform, mut prank) = pranks.get_mut(entity).expect("already checked");

    prank.pitch = (prank.pitch - prank.sensitivity.y * rotation.y * time.delta_seconds())
        .clamp(-consts::FRAC_PI_2, consts::FRAC_PI_2);
    prank.yaw -= prank.sensitivity.x * rotation.x * time.delta_seconds();

    transform.rotation = Quat::from_euler(EulerRot::YXZ, prank.yaw, prank.pitch, 0.0);
}
