use bevy::{prelude::*, render::camera::RenderTarget, window::WindowRef};

pub mod prelude;

pub struct PrankPlugin {
    default_movement_input: bool,
}

impl Default for PrankPlugin {
    fn default() -> Self {
        Self {
            default_movement_input: true,
        }
    }
}

impl Plugin for PrankPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PrankMovement>();
        app.register_type::<Prank3d>();

        if self.default_movement_input {
            app.add_systems(PreUpdate, movement_input);
        }

        app.add_systems(Update, movement);
    }
}

#[derive(Event)]
pub struct PrankMovement(pub Vec3);

fn movement_input(mut event: EventWriter<PrankMovement>, keyboard: Res<Input<KeyCode>>) {
    let mut movement = Vec3::ZERO;

    if keyboard.pressed(KeyCode::W) {
        movement += Vec3::NEG_Z;
    }
    if keyboard.pressed(KeyCode::A) {
        movement += Vec3::NEG_X;
    }
    if keyboard.pressed(KeyCode::S) {
        movement += Vec3::Z;
    }
    if keyboard.pressed(KeyCode::D) {
        movement += Vec3::X;
    }
    if keyboard.pressed(KeyCode::ShiftLeft) {
        movement = Vec3::new(movement.x, 0.0, movement.z);
    }
    if keyboard.pressed(KeyCode::E) {
        movement += Vec3::Y;
    }
    if keyboard.pressed(KeyCode::Q) {
        movement += Vec3::NEG_Y;
    }

    event.send(PrankMovement(movement.normalize_or_zero()));
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
    mut event: EventReader<PrankMovement>,
    time: Res<Time>,
) {
    for (mut transform, camera, prank) in pranks.iter_mut() {
        if !camera.is_active {
            continue;
        }
        let RenderTarget::Window(window) = camera.target else {
            continue;
        };
        let WindowRef::Primary = window else {
            continue;
        };

        let mut direction = Vec3::ZERO;
        for movement in event.iter() {
            direction += movement.0;
        }

        transform.translation += prank.speed * direction * time.delta_seconds();
        break;
    }
}
