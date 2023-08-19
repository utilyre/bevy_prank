//! Provides three-dimensional camera functionality.

use self::{
    gizmo::Prank3dGizmoPlugin,
    hud::Prank3dHudPlugin,
    state::{any_active, Prank3dActive, Prank3dState, Prank3dStatePlugin},
};
use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
};
use std::f32::consts;

pub mod gizmo;
pub mod hud;
mod state;

pub(super) struct Prank3dPlugin;

impl Plugin for Prank3dPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((Prank3dGizmoPlugin, Prank3dHudPlugin, Prank3dStatePlugin))
            .register_type::<Prank3d>()
            .add_systems(
                Update,
                (
                    initialize,
                    (
                        interpolation,
                        fly.run_if(in_state(Prank3dState::Fly)),
                        offset.run_if(in_state(Prank3dState::Offset)),
                    )
                        .run_if(any_active),
                ),
            );
    }
}

/// Adds debug functionality to [`Camera3dBundle`].
///
/// Once [`Prank3d`] is attached to an entity, its `rotation` field of [`Transform`] should not be
/// mutated manually.
///
/// # Example
///
/// ```
/// # use bevy::prelude::*;
/// # use bevy_prank::prelude::*;
/// #
/// fn setup(mut commands: Commands) {
///     commands.spawn((
///         Prank3d::default(),
///         Camera3dBundle::default(),
///     ));
/// }
/// #
/// # bevy::ecs::system::assert_is_system(setup);
/// ```
#[derive(Reflect, Component)]
#[reflect(Component)]
pub struct Prank3d {
    /// Whether user inputs should be applied to this [`Camera`].
    ///
    /// If more than one [`Camera`] with their `target` field set to the same window have this
    /// enabled, only one of them will be picked.
    pub is_active: bool,

    /// Constant speed that the [`Camera`] moves at.
    pub speed: f32,

    /// Scalar of `speed` field to adjust during gameplay with [`MouseWheel`].
    pub speed_scalar: f32,

    /// The rate that the [`Camera`] approaches its translation.
    ///
    /// Values closer to zero make the approaching faster.
    /// Zero disables interpolation.
    ///
    /// # Panic
    ///
    /// If its not in range `[0.0, 1.0)`.
    pub lerp_rate: f32,

    /// Sensitivity of [`MouseMotion`].
    pub sensitivity: Vec2,

    /// The current translation that the camera approaches towards.
    ///
    /// This should be used instead of [`Transform`]'s `translation` field, with the exception of
    /// initializing the [`Transform`] component.
    pub translation: Vec3,
}

impl Default for Prank3d {
    fn default() -> Self {
        Self {
            is_active: true,
            speed: 25.0,
            speed_scalar: 1.0,
            lerp_rate: 0.001,
            sensitivity: Vec2::splat(0.08),
            translation: Vec3::ZERO,
        }
    }
}

fn initialize(mut pranks: Query<(&mut Prank3d, &Transform), Added<Prank3d>>) {
    for (mut prank, transform) in pranks.iter_mut() {
        if !(0.0..1.0).contains(&prank.lerp_rate) {
            panic!("`lerp_rate` field of `bevy_prank::three::Prank3d` must be in range [0.0, 1.0)");
        }

        prank.translation = transform.translation;
    }
}

fn interpolation(
    active: Res<Prank3dActive>,
    mut pranks: Query<(&mut Transform, &Prank3d)>,
    time: Res<Time>,
) {
    let (mut transform, prank) = pranks.get_mut(active.expect("is active")).expect("exists");

    transform.translation = transform.translation.lerp(
        prank.translation,
        1.0 - prank.lerp_rate.powf(time.delta_seconds()),
    );
}

fn fly(
    active: Res<Prank3dActive>,
    mut pranks: Query<(&mut Transform, &mut Prank3d)>,
    time: Res<Time>,
    mut motion: EventReader<MouseMotion>,
    mut wheel: EventReader<MouseWheel>,
    keyboard: Res<Input<KeyCode>>,
) {
    let (mut transform, mut prank) = pranks.get_mut(active.expect("is active")).expect("exists");
    let motion = motion.iter().fold(Vec2::ZERO, |acc, m| acc + m.delta);
    let wheel = wheel.iter().fold(0.0, |acc, w| acc + w.y);
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

    prank.speed_scalar = (prank.speed_scalar + 0.1 * wheel).clamp(0.1, 10.0);

    let speed = prank.speed_scalar.powi(2) * prank.speed;
    prank.translation += speed * movement.normalize_or_zero() * time.delta_seconds();

    let (yaw, pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);
    transform.rotation = Quat::from_euler(
        EulerRot::YXZ,
        yaw - prank.sensitivity.x * motion.x * time.delta_seconds(),
        (pitch - prank.sensitivity.y * motion.y * time.delta_seconds())
            .clamp(-consts::FRAC_PI_3, consts::FRAC_PI_3),
        0.0,
    );
}

fn offset(
    active: Res<Prank3dActive>,
    mut pranks: Query<(&mut Transform, &mut Prank3d)>,
    time: Res<Time>,
    mut motion: EventReader<MouseMotion>,
) {
    let (mut transform, mut prank) = pranks.get_mut(active.expect("is active")).expect("exists");
    let motion = motion.iter().fold(Vec2::ZERO, |acc, m| acc + m.delta);

    let r = transform.rotation;
    transform.translation += r * Vec3::new(motion.x, -motion.y, 0.0) * time.delta_seconds();
    prank.translation = transform.translation;
}
