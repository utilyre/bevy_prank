//! Provides three-dimensional hint gizmo.

use super::{any_active, Prank3d, Prank3dActive};
use crate::PrankConfig;
use bevy::{ecs::query::Has, prelude::*};

pub(super) struct Prank3dGizmoPlugin;

impl Plugin for Prank3dGizmoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (camera, point_light, spot_light).run_if(any_active));
    }
}

/// Three-dimensional hint gizmo configuration.
#[derive(Clone, Reflect)]
pub struct Prank3dGizmoConfig {
    /// [`Camera`] spherical gizmo radius.
    pub camera_radius: f32,

    /// [`Camera`] gizmo color.
    pub camera_color: Color,

    /// [`Prank3d`]  gizmo color.
    pub prank_color: Color,

    /// [`PointLight`] gizmo radius.
    pub point_light_radius: f32,

    /// [`PointLight`] gizmo color.
    pub point_light_color: Color,

    /// [`SpotLight`] gizmo radius.
    pub spot_light_radius: f32,

    /// [`SpotLight`] gizmo color.
    pub spot_light_color: Color,
}

impl Default for Prank3dGizmoConfig {
    fn default() -> Self {
        Self {
            camera_radius: 1.0,
            camera_color: Color::CYAN,
            prank_color: Color::PINK,
            point_light_radius: 0.25,
            point_light_color: Color::WHITE,
            spot_light_radius: 0.25,
            spot_light_color: Color::WHITE,
        }
    }
}

fn camera(
    mut gizmos: Gizmos,
    config: Res<PrankConfig>,
    active: Res<Prank3dActive>,
    cameras: Query<(Entity, &GlobalTransform, Has<Prank3d>), With<Camera>>,
) {
    let Some(config) = config.gizmo3d.clone() else {
        return;
    };
    let prank_entity = active.0.expect("is active");

    for (camera_entity, camera_transform, camera_has_prank) in cameras.iter() {
        if camera_entity == prank_entity {
            continue;
        }

        let color = if camera_has_prank {
            config.prank_color
        } else {
            config.camera_color
        };

        let (_, rotation, translation) = camera_transform.to_scale_rotation_translation();
        gizmos.sphere(translation, rotation, config.camera_radius, color);
        gizmos.ray(
            translation,
            config.camera_radius * camera_transform.forward(),
            color,
        );
    }
}

fn point_light(
    mut gizmos: Gizmos,
    config: Res<PrankConfig>,
    point_lights: Query<&GlobalTransform, With<PointLight>>,
) {
    let Some(config) = config.gizmo3d.clone() else {
        return;
    };

    for transform in point_lights.iter() {
        let (_, rotation, translation) = transform.to_scale_rotation_translation();
        gizmos.sphere(
            translation,
            rotation,
            config.point_light_radius,
            config.point_light_color,
        );
    }
}

fn spot_light(
    mut gizmos: Gizmos,
    config: Res<PrankConfig>,
    spot_lights: Query<&GlobalTransform, With<SpotLight>>,
) {
    let Some(config) = config.gizmo3d.clone() else {
        return;
    };

    for transform in spot_lights.iter() {
        let (_, rotation, translation) = transform.to_scale_rotation_translation();
        gizmos.sphere(
            translation,
            rotation,
            config.spot_light_radius,
            config.spot_light_color,
        );
        gizmos.ray(
            translation,
            config.spot_light_radius * transform.forward(),
            config.spot_light_color,
        );
    }
}
