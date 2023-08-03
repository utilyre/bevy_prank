use bevy::prelude::*;
use three::Prank3dPlugin;

pub mod prelude;
pub mod three;

pub struct PrankPlugin {
    pub default_direction_input: bool,
    pub default_rotation_input: bool,
}

impl Default for PrankPlugin {
    fn default() -> Self {
        Self {
            default_direction_input: true,
            default_rotation_input: true,
        }
    }
}

impl Plugin for PrankPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Prank3dPlugin {
            default_direction_input: self.default_direction_input,
            default_rotaion_input: self.default_rotation_input,
        });
    }
}
