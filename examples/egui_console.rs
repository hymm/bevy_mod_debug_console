// pausing game loop does not work with egui_console
// press the GRAVE key to open the console

use bevy::prelude::*;
use bevy::{
    ecs::{archetype::Archetypes, component::Components, entity::Entities},
    reflect::TypeRegistry,
};
use bevy_console::{
    ConsoleCommandEntered, ConsoleConfiguration, ConsolePlugin, FromValue, PrintConsoleLine,
};
use bevy_mod_debug_console::{build_commands, match_commands, Pause};

#[derive(Component)]
struct Thing(String);

fn debug_console(
    mut console_events: EventReader<ConsoleCommandEntered>,
    mut console_line: EventWriter<PrintConsoleLine>,
    a: &Archetypes,
    c: &Components,
    e: &Entities,
    mut pause: ResMut<Pause>,
    reflect: Res<TypeRegistry>,
) {
    let app_name = "";
    for event in console_events.iter() {
        let console_app = build_commands(app_name);
        let mut args = vec![app_name];
        args.push(&event.command);

        let split: Vec<String> = event
            .args
            .iter()
            .filter_map(|x| String::from_value(x, 0).ok())
            .collect();
        args.append(&mut split.iter().map(|s| s.as_ref()).collect());

        let matches_result = console_app.try_get_matches_from(args);

        if let Err(e) = matches_result {
            console_line.send(PrintConsoleLine::new(e.to_string()));
            return;
        }

        let output = match_commands(&matches_result.unwrap(), a, c, e, &mut pause, &*reflect);

        console_line.send(PrintConsoleLine::new(output));
    }
}

fn setup(mut commands: Commands) {
    // Adds some Entities to test out `entities list` command
    commands.spawn().insert(Thing("Entity 1".to_string()));
    commands.spawn().insert(Thing("Entity 2".to_string()));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ConsoleConfiguration {
            // override config here
            ..Default::default()
        })
        .add_plugin(ConsolePlugin)
        .insert_resource(Pause(false))
        .add_startup_system(setup)
        .add_system(debug_console)
        .run();
}
