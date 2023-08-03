use bevy::prelude::*;
use three::Prank3dPlugin;

pub mod prelude;
pub mod three;

pub struct PrankPlugin;

impl Plugin for PrankPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Prank3dPlugin);
    }
}
