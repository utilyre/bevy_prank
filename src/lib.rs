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
        app.add_plugins(Prank3dPlugin {
            hud: self.hud.clone(),
        });
    }
}

#[derive(Clone, Resource)]
pub struct Prank3dHudConfig {
    pub height: Val,
    pub background_color: BackgroundColor,
    pub text_style: TextStyle,
}

impl Default for Prank3dHudConfig {
    fn default() -> Self {
        Self {
            height: Val::Px(25.0),
            background_color: Color::BLACK.with_a(0.9).into(),
            text_style: TextStyle {
                font_size: 14.0,
                color: Color::WHITE,
                ..default()
            },
        }
    }
}
