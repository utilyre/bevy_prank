#![allow(clippy::type_complexity)]
#![doc = include_str!("../README.md")]

use self::three::{gizmo::Prank3dGizmoConfig, hud::Prank3dHudConfig, Prank3dPlugin};
use bevy::prelude::*;

pub mod prelude;
pub mod three;

/// Opinionated Unreal Engine inspired spectator camera for the Bevy game engine.
///
/// # Example
///
/// Add [`PrankPlugin`] to your app.
///
/// ```
/// # use bevy::prelude::*;
/// # use bevy_prank::prelude::*;
/// #
/// App::new()
///     .add_plugins((DefaultPlugins, PrankPlugin::default()))
///     .run();
/// ```
#[derive(Default)]
pub struct PrankPlugin(pub PrankConfig);

impl Plugin for PrankPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Prank3dPlugin)
            .register_type::<PrankConfig>()
            .insert_resource(self.0.clone());
    }
}

/// Configuration of [`PrankPlugin`].
///
/// # Example
///
/// ```
/// # use bevy::prelude::*;
/// # use bevy_prank::prelude::*;
/// #
/// App::new()
///     .add_plugins((
///         DefaultPlugins,
///         PrankPlugin(PrankConfig {
///             hud: None,
///            ..default()
///         }),
///     ))
///     .run();
/// ```
#[derive(Clone, Reflect, Resource)]
#[reflect(Resource)]
pub struct PrankConfig {
    /// Three-dimensional hint gizmo configuration.
    ///
    /// Set `None` to disable.
    pub gizmo3d: Option<Prank3dGizmoConfig>,

    /// Three-dimensional camera HUD overlay configuration.
    ///
    /// Set `None` to disable.
    pub hud3d: Option<Prank3dHudConfig>,
}

impl Default for PrankConfig {
    fn default() -> Self {
        Self {
            gizmo3d: Some(Prank3dGizmoConfig::default()),
            hud3d: Some(Prank3dHudConfig::default()),
        }
    }
}
