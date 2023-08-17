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
#[derive(Clone, Resource)]
pub struct PrankConfig {
    /// Hint gizmos configuration.
    ///
    /// Set `None` to disable gizmo.
    pub gizmo: Option<Prank3dGizmoConfig>,

    /// Camera HUD overlay configuration.
    ///
    /// Set `None` to disable HUD.
    pub hud: Option<Prank3dHudConfig>,
}

impl Default for PrankConfig {
    fn default() -> Self {
        Self {
            hud: Some(Prank3dHudConfig::default()),
            gizmo: Some(Prank3dGizmoConfig::default()),
        }
    }
}
