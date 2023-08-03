use self::input::{
    Prank3dInputPlugin, Prank3dMode, Prank3dMovement, Prank3dRotation, Prank3dSpeedFactor,
};
use bevy::{
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};
use std::f32::consts;

mod input;

pub(super) struct Prank3dPlugin;

impl Plugin for Prank3dPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Prank3dInputPlugin);

        app.init_resource::<Prank3dActive>();
        app.register_type::<Prank3d>();

        app.add_systems(PreUpdate, sync_active);
        app.add_systems(
            Update,
            (
                spawn_speed_factor_text
                    .run_if(|active: Res<Prank3dActive>| active.is_changed() && active.0.is_some()),
                despawn_speed_factor_text
                    .run_if(|active: Res<Prank3dActive>| active.is_changed() && active.0.is_none()),
                sync_speed_factor_text
                    .run_if(|speed_factor: Res<Prank3dSpeedFactor>| speed_factor.is_changed()),
                sync_cursor.run_if(|mode: Res<State<Prank3dMode>>| mode.is_changed()),
                movement.run_if(not(in_state(Prank3dMode::None))),
                rotation.run_if(in_state(Prank3dMode::Fly)),
            ),
        );
    }
}

#[derive(Default, Resource)]
struct Prank3dActive(Option<Entity>);

#[derive(Reflect, Component)]
#[reflect(Component)]
pub struct Prank3d {
    pub speed: f32,
    pub sensitivity: Vec2,
    pub pitch: f32,
    pub yaw: f32,
}

impl Default for Prank3d {
    fn default() -> Self {
        Self {
            speed: 10.0,
            sensitivity: Vec2::splat(0.08),
            pitch: 0.0,
            yaw: 0.0,
        }
    }
}

#[derive(Component)]
struct SpeedFactorText;

fn sync_active(pranks: Query<(Entity, &Camera), With<Prank3d>>, mut active: ResMut<Prank3dActive>) {
    let new = pranks
        .iter()
        .find(|(_, camera)| camera.is_active)
        .map(|(entity, _)| entity);
    if new == active.0 {
        return;
    }

    *active = Prank3dActive(new);
}

fn spawn_speed_factor_text(
    mut commands: Commands,
    speed_factor_text: Query<(), With<SpeedFactorText>>,
    speed_factor: Res<Prank3dSpeedFactor>,
) {
    if !speed_factor_text.is_empty() {
        return;
    }

    commands.spawn((
        Name::new("SpeedFactorText"),
        SpeedFactorText,
        TextBundle::from_section(
            format!("{:.1}", speed_factor.0),
            TextStyle {
                font_size: 18.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(8.0),
            right: Val::Px(24.0),
            ..default()
        }),
    ));
}

fn despawn_speed_factor_text(
    mut commands: Commands,
    speed_factor_text: Query<Entity, With<SpeedFactorText>>,
) {
    let Ok(entity) = speed_factor_text.get_single() else {
        return;
    };

    commands.entity(entity).despawn_recursive();
}

fn sync_speed_factor_text(
    mut speed_factor_text: Query<&mut Text, With<SpeedFactorText>>,
    speed_factor: Res<Prank3dSpeedFactor>,
) {
    let Ok(mut text) = speed_factor_text.get_single_mut() else {
        return;
    };

    text.sections[0].value = format!("{:.1}", speed_factor.0);
}

fn sync_cursor(mut window: Query<&mut Window, With<PrimaryWindow>>, mode: Res<State<Prank3dMode>>) {
    let mut window = window.single_mut();

    match **mode {
        Prank3dMode::Fly => {
            window.cursor.visible = false;
            window.cursor.grab_mode = CursorGrabMode::Locked;
        }
        Prank3dMode::Offset => {
            window.cursor.visible = false;
            window.cursor.grab_mode = CursorGrabMode::Locked;
        }
        Prank3dMode::None => {
            window.cursor.visible = true;
            window.cursor.grab_mode = CursorGrabMode::None;
        }
    }
}

fn movement(
    mut movement: EventReader<Prank3dMovement>,
    active: Res<Prank3dActive>,
    mut pranks: Query<(&mut Transform, &Prank3d)>,
    time: Res<Time>,
) {
    let movement = movement.iter().fold(Vec3::ZERO, |acc, x| acc + x.0);
    let Some(entity) = active.0 else {
        return;
    };
    let (mut transform, prank) = pranks.get_mut(entity).expect("already checked");

    transform.translation += prank.speed * movement * time.delta_seconds();
}

fn rotation(
    mut rotation: EventReader<Prank3dRotation>,
    active: Res<Prank3dActive>,
    mut pranks: Query<(&mut Transform, &mut Prank3d)>,
    time: Res<Time>,
) {
    let rotation = rotation.iter().fold(Vec2::ZERO, |acc, x| acc + x.0);
    let Some(entity) = active.0 else {
        return;
    };
    let (mut transform, mut prank) = pranks.get_mut(entity).expect("already checked");

    prank.pitch = (prank.pitch - prank.sensitivity.y * rotation.y * time.delta_seconds())
        .clamp(-consts::FRAC_PI_2, consts::FRAC_PI_2);
    prank.yaw -= prank.sensitivity.x * rotation.x * time.delta_seconds();

    transform.rotation = Quat::from_euler(EulerRot::YXZ, prank.yaw, prank.pitch, 0.0);
}
