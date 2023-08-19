use super::Prank3d;
use bevy::{
    prelude::*,
    render::camera::{NormalizedRenderTarget, RenderTarget},
    window::{CursorGrabMode, PrimaryWindow, WindowRef},
};

pub(super) struct Prank3dStatePlugin;

impl Plugin for Prank3dStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<Prank3dState>()
            .init_resource::<Prank3dActive>()
            .add_systems(
                PreUpdate,
                (
                    sync_active,
                    sync_state
                        .after(sync_active)
                        .run_if(resource_changed::<Prank3dActive>().or_else(any_active_prank)),
                ),
            )
            .add_systems(
                Update,
                sync_cursor.run_if(any_active_prank.and_then(
                    resource_changed::<Prank3dActive>().or_else(state_changed::<Prank3dState>()),
                )),
            );
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, States)]
pub(super) enum Prank3dState {
    Fly,
    Offset,
    #[default]
    None,
}

#[derive(Default, Resource)]
pub(super) struct Prank3dActive(pub(super) Option<Entity>);

pub(super) fn any_active_prank(active: Res<Prank3dActive>) -> bool {
    active.0.is_some()
}

fn sync_active(
    primary_window: Query<(Entity, &Window), With<PrimaryWindow>>,
    windows: Query<(Entity, &Window), Without<PrimaryWindow>>,
    pranks: Query<(Entity, &Camera, &Prank3d)>,
    mut active: ResMut<Prank3dActive>,
) {
    let primary_window = primary_window.get_single().ok();
    let Some(focused_window) = windows
        .iter()
        .find(|(_, window)| window.focused)
        .map(|(entity, _)| entity)
        .or_else(|| primary_window.and_then(|(entity, window)| window.focused.then_some(entity)))
    else {
        return;
    };

    let active_entity = pranks
        .iter()
        .find(|(_, camera, prank)| {
            if !prank.is_active {
                return false;
            }
            let Some(NormalizedRenderTarget::Window(winref)) = camera
                .target
                .normalize(primary_window.map(|(entity, _)| entity))
            else {
                return false;
            };

            winref.entity() == focused_window
        })
        .map(|(entity, _, _)| entity);

    if active_entity != active.0 {
        *active = Prank3dActive(active_entity);
    }
}

fn sync_state(
    active: Res<Prank3dActive>,
    prev_state: Res<State<Prank3dState>>,
    mut state: ResMut<NextState<Prank3dState>>,
    mouse: Res<Input<MouseButton>>,
) {
    if active.0.is_none() {
        state.set(Prank3dState::None);
        return;
    }

    match **prev_state {
        Prank3dState::Fly => {
            if !mouse.pressed(MouseButton::Right) {
                state.set(Prank3dState::None);
            }
        }
        Prank3dState::Offset => {
            if !mouse.pressed(MouseButton::Middle) {
                state.set(Prank3dState::None);
            }
        }
        Prank3dState::None => {
            if mouse.pressed(MouseButton::Right) {
                state.set(Prank3dState::Fly);
            } else if mouse.pressed(MouseButton::Middle) {
                state.set(Prank3dState::Offset);
            }
        }
    }
}

fn sync_cursor(
    mut primary_window: Query<&mut Window, With<PrimaryWindow>>,
    mut windows: Query<&mut Window, Without<PrimaryWindow>>,
    active: Res<Prank3dActive>,
    pranks: Query<&Camera, With<Prank3d>>,
    state: Res<State<Prank3dState>>,
) {
    let camera = pranks.get(active.0.expect("is active")).expect("exists");
    let RenderTarget::Window(winref) = camera.target else {
        return;
    };
    let Some(mut window) = (match winref {
        WindowRef::Primary => primary_window.get_single_mut().ok(),
        WindowRef::Entity(entity) => windows.get_mut(entity).ok(),
    }) else {
        return;
    };

    match **state {
        Prank3dState::Fly => {
            window.cursor.visible = false;
            window.cursor.grab_mode = CursorGrabMode::Locked;
        }
        Prank3dState::Offset => {
            window.cursor.visible = false;
            window.cursor.grab_mode = CursorGrabMode::Locked;
        }
        Prank3dState::None => {
            window.cursor.visible = true;
            window.cursor.grab_mode = CursorGrabMode::None;
        }
    }
}
