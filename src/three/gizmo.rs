//! Hint gizmos.

use super::{Prank3d, Prank3dActive};
use crate::PrankConfig;
use bevy::{ecs::query::Has, prelude::*};

pub(super) struct Prank3dGizmoPlugin;

impl Plugin for Prank3dGizmoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, cameras);
    }
}

/// Hint gizmos configuration.
#[derive(Clone)]
pub struct Prank3dGizmoConfig {
    /// Camera spherical gizmo radius.
    pub camera_radius: f32,

    /// Ordinary camera gizmo color.
    pub camera_color: Color,

    /// Prank camera gizmo color.
    pub prank_color: Color,
}

impl Default for Prank3dGizmoConfig {
    fn default() -> Self {
        Self {
            camera_radius: 1.0,
            camera_color: Color::CYAN,
            prank_color: Color::PINK,
        }
    }
}

fn cameras(
    mut gizmos: Gizmos,
    config: Res<PrankConfig>,
    active: Res<Prank3dActive>,
    cameras: Query<(Entity, &GlobalTransform, Has<Prank3d>), With<Camera3d>>,
) {
    let Some(config) = config.gizmo.clone() else {
        return;
    };
    let Some(prank_entity) = active.0 else {
        return;
    };

    for (camera_entity, camera_transform, has_prank) in cameras.iter() {
        if camera_entity == prank_entity {
            continue;
        }

        let color = if has_prank {
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
