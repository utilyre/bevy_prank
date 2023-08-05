#![allow(clippy::type_complexity)]
#![doc = include_str!("../README.md")]

use self::three::hud::Prank3dHudConfig;
use self::three::Prank3dPlugin;
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
/// # use bevy::{prelude::*, diagnostic::FrameTimeDiagnosticsPlugin};
/// # use bevy_prank::prelude::*;
/// #
/// let mut app = App::new();
///
/// app.add_plugins((DefaultPlugins, FrameTimeDiagnosticsPlugin));
/// app.add_plugins(PrankPlugin::default());
///
/// app.run();
/// ```
#[derive(Default)]
pub struct PrankPlugin(pub PrankConfig);

impl Plugin for PrankPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.0.clone());

        app.add_plugins(Prank3dPlugin);
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
/// let mut app = App::new();
///
/// app.add_plugins(DefaultPlugins);
/// app.add_plugins(PrankPlugin(PrankConfig {
///     hud: None,
///     ..default()
/// }));
///
/// app.run();
/// ```
#[derive(Clone, Resource)]
pub struct PrankConfig {
    /// Camera HUD overlay configuration.
    ///
    /// Set `None` to disable HUD.
    pub hud: Option<Prank3dHudConfig>,
}

impl Default for PrankConfig {
    fn default() -> Self {
        Self {
            hud: Some(Prank3dHudConfig::default()),
        }
    }
}
