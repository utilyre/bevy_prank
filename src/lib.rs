use bevy::prelude::*;
use three::Prank3dPlugin;

pub mod prelude;
pub mod three;

pub struct PrankPlugin {
    pub hud: Option<Prank3dHudConfig>,
}

impl Default for PrankPlugin {
    fn default() -> Self {
        Self {
            hud: Some(Prank3dHudConfig::default()),
        }
    }
}

impl Plugin for PrankPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(match self.hud {
            Some(ref config) => config.clone(),
            None => Prank3dHudConfig::default(),
        });

        app.add_plugins(Prank3dPlugin);
    }
}

#[derive(Clone, Resource)]
pub struct Prank3dHudConfig {
    pub background_color: BackgroundColor,
    pub text_style: TextStyle,
}

impl Default for Prank3dHudConfig {
    fn default() -> Self {
        Self {
            background_color: Color::BLACK.with_a(0.9).into(),
            text_style: TextStyle {
                font_size: 14.0,
                color: Color::WHITE,
                ..default()
            },
        }
    }
}
