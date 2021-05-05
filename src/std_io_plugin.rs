use crate::app::{build_commands, input_pause, match_commands, pause, EnteringConsole, Pause};
use bevy::{
    ecs::{archetype::Archetypes, component::Components, entity::Entities},
    prelude::*,
    reflect::TypeRegistry,
};
use std::io::{self, BufRead, Write};

fn parse_input(
    a: &Archetypes,
    c: &Components,
    e: &Entities,
    reflect: Res<TypeRegistry>,
    entering_console: Res<EnteringConsole>,
    mut pause: ResMut<Pause>,
) {
    if entering_console.0 {
        println!("Bevy Console Debugger.  Type 'help' for list of commands.");
    }
    print!(">>> ");
    io::stdout().flush().unwrap();
    let app_name = "";
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();

    println!("");
    let split = line.split_whitespace();
    let mut args = vec![app_name];
    args.append(&mut split.collect());

    let matches_result = build_commands(app_name).try_get_matches_from(args);

    if let Err(e) = matches_result {
        println!("{}", e.to_string());
        return;
    }

    let matches = matches_result.unwrap();

    let output = match_commands(&matches, a, c, e, &mut pause, &*reflect);

    println!("{}", output);
}

pub struct ConsoleDebugPlugin;
impl Plugin for ConsoleDebugPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(Pause(false))
            .insert_resource(EnteringConsole(false))
            .add_system(input_pause.system())
            .add_system(parse_input.system().with_run_criteria(pause.system()));
    }
}
