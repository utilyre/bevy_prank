//! Provides three-dimensional camera functionality.

use self::{gizmo::Prank3dGizmoPlugin, hud::Prank3dHudPlugin};
use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
    render::camera::NormalizedRenderTarget,
    window::{CursorGrabMode, PrimaryWindow},
};
use std::f32::consts;

pub mod gizmo;
pub mod hud;

pub(super) struct Prank3dPlugin;

impl Plugin for Prank3dPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((Prank3dGizmoPlugin, Prank3dHudPlugin))
            .register_type::<Prank3d>()
            .init_resource::<Prank3dActive>()
            .add_state::<Prank3dMode>()
            .add_systems(
                PreUpdate,
                (
                    sync_active,
                    mode.run_if(resource_changed::<Prank3dActive>().or_else(any_active_prank))
                        .after(sync_active),
                ),
            )
            .add_systems(
                Update,
                (
                    initialize,
                    sync_cursor.run_if(any_active_prank.and_then(
                        resource_changed::<Prank3dActive>().or_else(state_changed::<Prank3dMode>()),
                    )),
                    interpolation,
                    fly.run_if(in_state(Prank3dMode::Fly)),
                    offset.run_if(in_state(Prank3dMode::Offset)),
                ),
            );
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, States)]
enum Prank3dMode {
    Fly,
    Offset,
    #[default]
    None,
}

#[derive(Default, Resource)]
struct Prank3dActive(Option<Entity>);

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

fn any_active_prank(active: Res<Prank3dActive>) -> bool {
    active.0.is_some()
}

fn sync_active(
    primary_window: Query<(Entity, &Window), With<PrimaryWindow>>,
    windows: Query<(Entity, &Window), Without<PrimaryWindow>>,
    pranks: Query<(Entity, &Camera, &Prank3d)>,
    mut active: ResMut<Prank3dActive>,
) {
    let primary_window = primary_window.get_single().ok();
    let Some(focused_window) = windows
        .iter()
        .find(|(_, window)| window.focused)
        .map(|(entity, _)| entity)
        .or_else(|| primary_window.and_then(|(entity, window)| window.focused.then_some(entity)))
    else {
        return;
    };

    let active_entity = pranks
        .iter()
        .find(|(_, camera, prank)| {
            if !prank.is_active {
                return false;
            }
            let Some(NormalizedRenderTarget::Window(winref)) = camera
                .target
                .normalize(primary_window.map(|(entity, _)| entity))
            else {
                return false;
            };

            winref.entity() == focused_window
        })
        .map(|(entity, _, _)| entity);

    if active_entity != active.0 {
        *active = Prank3dActive(active_entity);
    }
}

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

fn initialize(mut pranks: Query<(&mut Prank3d, &Transform), Added<Prank3d>>) {
    for (mut prank, transform) in pranks.iter_mut() {
        if !(0.0..1.0).contains(&prank.lerp_rate) {
            panic!("`lerp_rate` field of `bevy_prank::three::Prank3d` must be in range [0.0, 1.0)");
        }

        prank.translation = transform.translation;
    }
}

fn sync_cursor(
    mut primary_window: Query<(Entity, &mut Window), With<PrimaryWindow>>,
    mut windows: Query<&mut Window, Without<PrimaryWindow>>,
    active: Res<Prank3dActive>,
    pranks: Query<&Camera, With<Prank3d>>,
    mode: Res<State<Prank3dMode>>,
) {
    let camera = pranks.get(active.0.expect("is active")).expect("exists");

    let Some(NormalizedRenderTarget::Window(winref)) = camera
        .target
        .normalize(primary_window.get_single().ok().map(|(entity,_)| entity))
    else {
        return;
    };

    let mut window = match windows.get_mut(winref.entity()) {
        Ok(window) => window,
        Err(_) => {
            let Ok((_, window)) = primary_window.get_single_mut() else {
                return;
            };

            window
        }
    };

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

fn interpolation(
    active: Res<Prank3dActive>,
    mut pranks: Query<(&mut Transform, &Prank3d)>,
    time: Res<Time>,
) {
    let Some(entity) = active.0 else {
        return;
    };
    let (mut transform, prank) = pranks.get_mut(entity).expect("exists");

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
    let Some(entity) = active.0 else {
        return;
    };
    let (mut transform, mut prank) = pranks.get_mut(entity).expect("exists");
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
    let Some(entity) = active.0 else {
        return;
    };
    let (mut transform, mut prank) = pranks.get_mut(entity).expect("exists");
    let motion = motion.iter().fold(Vec2::ZERO, |acc, m| acc + m.delta);

    let r = transform.rotation;
    transform.translation += r * Vec3::new(motion.x, -motion.y, 0.0) * time.delta_seconds();
    prank.translation = transform.translation;
}
