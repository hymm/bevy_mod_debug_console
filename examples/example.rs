use bevy::prelude::*;
use bevy_mod_debug_console::ConsoleDebugPlugin;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(ConsoleDebugPlugin)
        .run();
}
