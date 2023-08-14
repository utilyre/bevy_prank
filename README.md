<div align="center">

# [Bevy Prank](https://crates.io/crates/bevy_prank)

Opinionated Unreal Engine inspired spectator camera for the Bevy game engine.

</div>

[demo.webm](https://github.com/utilyre/bevy_prank/assets/91974155/fd971418-b369-49ff-b959-2985c92e5d62)

## ✨ Features

- Provides smooth interpolation for movement without sacrificing camera's translation.

- Includes an optional HUD overlay to display important information like FOV and FPS.

- Automatically grabs cursor when <kbd>MMB</kbd>/<kbd>RMB</kbd> is held.

- Allows complete control over movement via <kbd>LShift</kbd> prefixed controls.

## 📦 Installation

1.  Install the crate using `cargo`

    ```bash
    cargo add bevy_prank
    ```

2.  Add `PrankPlugin` to your app

    ```rust
    use bevy::prelude::*;
    use bevy_prank::prelude::*;

    fn main() {
        App::new()
            // ...
            .add_plugins((DefaultPlugins, PrankPlugin::default()))
            // ...
            .run();
    }
    ```

## 🚀 Usage

Spawn a `Camera3dBundle` along with a `Prank3d` component

```rust
use bevy::prelude::*;
use bevy_prank::prelude::*;

fn setup(mut commands: Commands) {
    commands.spawn((
        Prank3d::default(),
        Camera3dBundle::default(),
    ));
}
```

For further information see [examples][examples].

## 🎮 Controls

| Control                                                                                                            | Action                                                                                     |
| ------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------ |
| <kbd>MMB</kbd> + Drag                                                                                              | Offsets the camera on its local `x` (left/right) and `y` (top/bottom) axes                 |
| <kbd>RMB</kbd> + Drag                                                                                              | Rotates the camera                                                                         |
| <kbd>RMB</kbd> + Scroll                                                                                            | Adjusts movement speed                                                                     |
| <kbd>RMB</kbd> + <kbd>W</kbd> <kbd>A</kbd> <kbd>S</kbd> <kbd>D</kbd>                                               | Moves the camera on its local `x` (left/right) and `z` (front/back) axes                   |
| <kbd>RMB</kbd> + <kbd>E</kbd> <kbd>Q</kbd>                                                                         | Moves the camera on the `y` (top/bottom) axis                                              |
| <kbd>RMB</kbd> + <kbd>LShift</kbd> + <kbd>W</kbd> <kbd>A</kbd> <kbd>S</kbd> <kbd>D</kbd> <kbd>E</kbd> <kbd>Q</kbd> | Moves the camera on the `x` (left/right), `y` (top/bottom) axes, and `z` (front/back) axes |

[examples]: https://github.com/utilyre/bevy_prank/tree/main/examples
