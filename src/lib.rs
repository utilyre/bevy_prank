use self::three::hud::Prank3dHudConfig;
use self::three::Prank3dPlugin;
use bevy::prelude::*;

pub mod prelude;
pub mod three;

#[derive(Clone, Resource)]
pub struct PrankConfig {
    pub hud: Option<Prank3dHudConfig>,
}

impl Default for PrankConfig {
    fn default() -> Self {
        Self {
            hud: Some(Prank3dHudConfig::default()),
        }
    }
}

#[derive(Default)]
pub struct PrankPlugin(pub PrankConfig);

impl Plugin for PrankPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.0.clone());

        app.add_plugins(Prank3dPlugin);
    }
}
