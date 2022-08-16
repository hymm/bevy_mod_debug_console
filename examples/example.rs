use bevy::prelude::*;
use bevy_mod_debug_console::ConsoleDebugPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ConsoleDebugPlugin)
        .run();
}