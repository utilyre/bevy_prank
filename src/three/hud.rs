//! Provides three-dimensional camera HUD overlay.

use super::{any_active, Prank3d, Prank3dActive};
use crate::PrankConfig;
use bevy::prelude::*;

pub(super) struct Prank3dHudPlugin;

impl Plugin for Prank3dHudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn.run_if(just_activated),
                despawn.run_if(just_inactivated),
                (sync_translation, sync_fps, sync_fov, sync_speed).run_if(any_active),
            ),
        );
    }
}

/// Three-dimensional camera HUD overlay configuration.
#[derive(Clone, Reflect)]
pub struct Prank3dHudConfig {
    /// Overlay height.
    pub height: Val,

    /// Overlay background color.
    pub background_color: BackgroundColor,

    /// Overlay text style.
    pub text_style: TextStyle,
}

impl Default for Prank3dHudConfig {
    fn default() -> Self {
        Self {
            height: Val::Px(25.0),
            background_color: Color::BLACK.with_a(0.5).into(),
            text_style: TextStyle {
                font_size: 14.0,
                color: Color::WHITE,
                ..default()
            },
        }
    }
}

#[derive(Component)]
struct Hud;

#[derive(Component)]
struct HudTranslation;

#[derive(Component)]
struct HudFps;

#[derive(Component)]
struct HudFov;

#[derive(Component)]
struct HudSpeed;

fn just_activated(active: Res<Prank3dActive>) -> bool {
    active.is_changed() && active.is_some()
}

fn just_inactivated(active: Res<Prank3dActive>) -> bool {
    active.is_changed() && active.is_none()
}

fn spawn(mut commands: Commands, hud: Query<(), With<Hud>>, config: Res<PrankConfig>) {
    if !hud.is_empty() {
        return;
    }
    let Some(config) = config.hud3d.clone() else {
        return;
    };

    commands
        .spawn((
            Name::new("Hud"),
            Hud,
            NodeBundle {
                background_color: config.background_color,
                style: Style {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(0.0),
                    width: Val::Vw(100.0),
                    height: config.height,
                    padding: UiRect::horizontal(Val::Px(5.0)),
                    column_gap: Val::Px(20.0),
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Name::new("HudTranslation"),
                HudTranslation,
                TextBundle::from_section("", config.text_style.clone()),
            ));

            parent.spawn((
                Name::new("HudFps"),
                HudFps,
                TextBundle::from_section("", config.text_style.clone()),
            ));

            parent.spawn((
                Name::new("HudFov"),
                HudFov,
                TextBundle::from_section("", config.text_style.clone()),
            ));

            parent.spawn((
                Name::new("HudSpeed"),
                HudSpeed,
                TextBundle::from_section("", config.text_style.clone()),
            ));
        });
}

fn despawn(mut commands: Commands, hud: Query<Entity, With<Hud>>) {
    let Ok(entity) = hud.get_single() else {
        return;
    };

    commands.entity(entity).despawn_recursive();
}

fn sync_translation(
    mut hud_translation: Query<&mut Text, With<HudTranslation>>,
    active: Res<Prank3dActive>,
    pranks: Query<&Prank3d>,
) {
    let Ok(mut text) = hud_translation.get_single_mut() else {
        return;
    };
    let prank = pranks.get(active.expect("is active")).expect("exists");

    let Vec3 { x, y, z } = prank.translation;
    text.sections[0].value = format!("Translation: [{:.2}, {:.2}, {:.2}]", x, y, z);
}

fn sync_fps(mut hud_fps: Query<&mut Text, With<HudFps>>, time: Res<Time>) {
    let Ok(mut text) = hud_fps.get_single_mut() else {
        return;
    };

    text.sections[0].value = format!("FPS: {:.0}", time.delta_seconds().recip());
}

fn sync_fov(
    mut hud_fov: Query<&mut Text, With<HudFov>>,
    active: Res<Prank3dActive>,
    pranks: Query<&Projection, With<Prank3d>>,
) {
    let Ok(mut text) = hud_fov.get_single_mut() else {
        return;
    };
    let projection = pranks.get(active.expect("is active")).expect("exists");

    text.sections[0].value = match projection {
        Projection::Perspective(projection) => format!("FOV: {:.0}", projection.fov.to_degrees()),
        Projection::Orthographic(projection) => format!("SCALE: {:.2}", projection.scale),
    };
}

fn sync_speed(
    mut hud_speed: Query<&mut Text, With<HudSpeed>>,
    active: Res<Prank3dActive>,
    pranks: Query<&Prank3d>,
) {
    let Ok(mut text) = hud_speed.get_single_mut() else {
        return;
    };
    let prank = pranks.get(active.expect("is active")).expect("exists");

    text.sections[0].value = format!("Speed Scalar: {:.1}", prank.speed_scalar);
}
