<div align="center">

# [Bevy Prank](https://crates.io/crates/bevy_prank)

Opinionated Unreal Engine inspired spectator camera for the Bevy game engine.

</div>

[demo.webm](https://github.com/utilyre/bevy_prank/assets/91974155/fd971418-b369-49ff-b959-2985c92e5d62)

## 📦 Installation

1.  Install the crate using `cargo`

    ```bash
    cargo add bevy_prank
    ```

2.  Add `PrankPlugin` to your app

    ```rust
    use bevy::{prelude::*, diagnostic::FrameTimeDiagnosticsPlugin};
    use bevy_prank::prelude::*;

    fn main() {
        let mut app = App::new();

        // ...

        app.add_plugins((
            DefaultPlugins,
            // required unless `hud` field of PrankPlugin is set to `None`
            FrameTimeDiagnosticsPlugin,
        ));
        app.add_plugins(PrankPlugin::default());

        // ...

        app.run();
    }
    ```

## 🎮 Controls

| Control                                                                                                            | Action                                                                                     |
| ------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------ |
| <kbd>MMD</kbd> + Drag                                                                                              | Offsets the camera on its local `x` (left/right) and `y` (top/bottom) axes                 |
| <kbd>RMB</kbd> + Drag                                                                                              | Rotates the camera                                                                         |
| <kbd>RMB</kbd> + <kbd>W</kbd> <kbd>A</kbd> <kbd>S</kbd> <kbd>D</kbd>                                               | Moves the camera on its local `x` (left/right) and `z` (front/back) axes                   |
| <kbd>RMB</kbd> + <kbd>E</kbd> <kbd>Q</kbd>                                                                         | Moves the camera on the `y` (top/bottom) axis                                              |
| <kbd>RMB</kbd> + <kbd>LShift</kbd> + <kbd>W</kbd> <kbd>A</kbd> <kbd>S</kbd> <kbd>D</kbd> <kbd>E</kbd> <kbd>Q</kbd> | Moves the camera on the `x` (left/right), `y` (top/bottom) axes, and `z` (front/back) axes |
