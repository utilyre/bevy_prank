use super::{Prank3d, Prank3dActive};
use crate::Prank3dHudConfig;
use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

pub(super) struct Prank3dHudPlugin;

impl Plugin for Prank3dHudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn
                    .run_if(|active: Res<Prank3dActive>| active.is_changed() && active.0.is_some()),
                despawn
                    .run_if(|active: Res<Prank3dActive>| active.is_changed() && active.0.is_none()),
                sync_position,
                sync_fps,
                sync_fov,
                sync_speed_factor,
            ),
        );
    }
}

#[derive(Component)]
struct Hud;

#[derive(Component)]
struct HudPosition;

#[derive(Component)]
struct HudFps;

#[derive(Component)]
struct HudFov;

#[derive(Component)]
struct HudSpeedFactor;

fn spawn(mut commands: Commands, hud: Query<(), With<Hud>>, config: Res<Prank3dHudConfig>) {
    if !hud.is_empty() {
        return;
    }

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
                    height: Val::Px(25.0),
                    padding: UiRect::all(Val::Px(2.0)),
                    column_gap: Val::Px(20.0),
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Name::new("HudPosition"),
                HudPosition,
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
                Name::new("HudSpeedFactor"),
                HudSpeedFactor,
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

fn sync_position(
    mut hud_position: Query<&mut Text, With<HudPosition>>,
    active: Res<Prank3dActive>,
    pranks: Query<&Prank3d>,
) {
    let Ok(mut text) = hud_position.get_single_mut() else {
        return;
    };
    let Some(entity) = active.0 else {
        return;
    };
    let Ok(prank) = pranks.get(entity) else {
        return;
    };

    let Vec3 { x, y, z } = prank.position;
    text.sections[0].value = format!("position: [{:.2}, {:.2}, {:.2}]", x, y, z);
}

fn sync_fps(mut hud_fps: Query<&mut Text, With<HudFps>>, diagnostics: Res<DiagnosticsStore>) {
    let Ok(mut text) = hud_fps.get_single_mut() else {
        return;
    };
    let Some(diagnostic) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) else {
        return;
    };
    let Some(fps) = diagnostic.smoothed() else {
        return;
    };

    text.sections[0].value = format!("fps: {:.0}", fps);
}

fn sync_fov(
    mut hud_fov: Query<&mut Text, With<HudFov>>,
    active: Res<Prank3dActive>,
    pranks: Query<&Projection, With<Prank3d>>,
) {
    let Ok(mut text) = hud_fov.get_single_mut() else {
        return;
    };
    let Some(entity) = active.0 else {
        return;
    };
    let Ok(projection) = pranks.get(entity) else {
        return;
    };

    text.sections[0].value = match projection {
        Projection::Perspective(projection) => format!("fov: {:.0}", projection.fov.to_degrees()),
        Projection::Orthographic(projection) => format!("scale: {:.2}", projection.scale),
    };
}

fn sync_speed_factor(
    active: Res<Prank3dActive>,
    pranks: Query<&Prank3d>,
    mut hud_speed_factor: Query<&mut Text, With<HudSpeedFactor>>,
) {
    let Ok(mut text) = hud_speed_factor.get_single_mut() else {
        return;
    };
    let Some(entity) = active.0 else {
        return;
    };
    let Ok(prank) = pranks.get(entity) else {
        return;
    };

    text.sections[0].value = format!("speed: {:.1}", prank.speed_factor);
}