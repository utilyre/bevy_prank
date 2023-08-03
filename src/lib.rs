use bevy::prelude::*;

pub mod prelude;

pub struct PrankPlugin {
    default_direction_input: bool,
}

impl Default for PrankPlugin {
    fn default() -> Self {
        Self {
            default_direction_input: true,
        }
    }
}

impl Plugin for PrankPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Prank3dDirection>();
        app.register_type::<Prank3d>();

        if self.default_direction_input {
            app.add_systems(PreUpdate, direction_input);
        }

        app.add_systems(Update, movement);
    }
}

#[derive(Event)]
pub struct Prank3dDirection(pub Vec3);

fn direction_input(mut event: EventWriter<Prank3dDirection>, keyboard: Res<Input<KeyCode>>) {
    let mut direction = Vec3::ZERO;

    if keyboard.pressed(KeyCode::W) {
        direction += Vec3::NEG_Z;
    }
    if keyboard.pressed(KeyCode::A) {
        direction += Vec3::NEG_X;
    }
    if keyboard.pressed(KeyCode::S) {
        direction += Vec3::Z;
    }
    if keyboard.pressed(KeyCode::D) {
        direction += Vec3::X;
    }
    if keyboard.pressed(KeyCode::ShiftLeft) {
        direction = Vec3::new(direction.x, 0.0, direction.z);
    }
    if keyboard.pressed(KeyCode::E) {
        direction += Vec3::Y;
    }
    if keyboard.pressed(KeyCode::Q) {
        direction += Vec3::NEG_Y;
    }

    event.send(Prank3dDirection(direction.normalize_or_zero()));
}

#[derive(Reflect, Component)]
#[reflect(Component)]
pub struct Prank3d {
    pub speed: f32,
}

impl Default for Prank3d {
    fn default() -> Self {
        Self { speed: 5.0 }
    }
}

fn movement(
    mut pranks: Query<(&mut Transform, &Camera, &Prank3d), With<Prank3d>>,
    mut direction: EventReader<Prank3dDirection>,
    time: Res<Time>,
) {
    let direction = direction.into_iter().fold(Vec3::ZERO, |acc, x| acc + x.0);

    for (mut transform, camera, prank) in pranks.iter_mut() {
        if !camera.is_active {
            continue;
        }

        transform.translation += prank.speed * direction * time.delta_seconds();
    }
}
