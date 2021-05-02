use crate::ecs;
use bevy::{
    ecs::{archetype::Archetypes, component::Components, entity::Entities, schedule::ShouldRun},
    prelude::{Input, KeyCode, Local, Res, ResMut},
};
use clap::{App, ArgMatches};
use std::process::exit;

pub fn build_commands<'a>(app_name: &'a str) -> App {
    let app = App::new(app_name);

    let app = build_app_commands(app);
    let app = ecs::build_commands(app);

    app
}

pub fn match_commands(
    matches: &ArgMatches,
    a: &Archetypes,
    c: &Components,
    e: &Entities,
    pause: &mut Pause,
) -> String {
    let mut output = String::new();

    output.push_str(&match_app_commands(matches, pause));
    output.push_str(&ecs::match_commands(matches, a, c, e));

    output
}

fn build_app_commands(app: App) -> App {
    let app = app
        .subcommand(App::new("resume").about("resume running game"))
        .subcommand(App::new("quit").about("quit game"));

    app
}

fn match_app_commands(matches: &ArgMatches, mut pause: &mut Pause) -> String {
    let mut output = String::new();
    match matches.subcommand() {
        Some(("resume", _)) => {
            pause.0 = false;
            output.push_str("...resuming game.");
        }
        Some(("quit", _)) => exit(0),
        _ => {}
    }

    output
}

#[derive(Default)]
pub struct Pause(pub bool);
pub struct EnteringConsole(pub bool);
pub fn pause(
    pause: Res<Pause>,
    mut last_pause: Local<Pause>,
    mut entering_console: ResMut<EnteringConsole>,
) -> ShouldRun {
    entering_console.0 = (pause.0 != last_pause.0) && pause.0;
    last_pause.0 = pause.0;
    if pause.0 {
        ShouldRun::YesAndCheckAgain
    } else {
        ShouldRun::No
    }
}

pub fn input_pause(keyboard_input: Res<Input<KeyCode>>, mut pause: ResMut<Pause>) {
    if keyboard_input.pressed(KeyCode::F10) {
        pause.0 = true;
    }
}
