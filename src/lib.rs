mod ecs;

use bevy::{ecs::schedule::ShouldRun, prelude::*};
use clap::{App, ArgMatches};
use std::io::{self, BufRead, Write};
use std::process::exit;
#[derive(Default)]
struct Pause(bool);

fn parse_input(world: &mut World) {
    let entering_console = world.get_resource::<EnteringConsole>().unwrap();
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

    let matches_result = build_app(app_name).try_get_matches_from(args);

    if let Err(e) = matches_result {
        println!("{}", e.to_string());
        return;
    }

    let matches = matches_result.unwrap();

    let output = match_commands(&matches, world);

    println!("{}", output);
}

fn build_app<'a>(app_name: &'a str) -> App {
    let app = App::new(app_name);

    let app = build_app_commands(app);
    let app = ecs::build_commands(app);

    app
}

fn match_commands(matches: &ArgMatches, world: &mut World) -> String {
    let mut output = String::new();

    output.push_str(&match_app_commands(matches, world));
    output.push_str(&ecs::match_commands(matches, world));

    output
}
struct EnteringConsole(bool);
fn pause(
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

fn input_pause(keyboard_input: Res<Input<KeyCode>>, mut pause: ResMut<Pause>) {
    if keyboard_input.pressed(KeyCode::F10) {
        pause.0 = true;
    }
}

fn build_app_commands(app: App) -> App {
    let app = app
        .subcommand(App::new("resume").about("resume running game"))
        .subcommand(App::new("quit").about("quit game"));

    app
}

fn match_app_commands(matches: &ArgMatches, world: &mut World) -> String {
    let mut output = String::new();
    match matches.subcommand() {
        Some(("resume", _)) => {
            let mut pause = world.get_resource_mut::<Pause>().unwrap();
            pause.0 = false;
            output.push_str("...resuming game.");
        }
        Some(("quit", _)) => exit(0),
        _ => {}
    }

    output
}

pub struct ConsoleDebugPlugin;
impl Plugin for ConsoleDebugPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(Pause(false))
            .insert_resource(EnteringConsole(false))
            .add_system(input_pause.system())
            .add_system(
                parse_input
                    .exclusive_system()
                    .with_run_criteria(pause.system()),
            );
    }
}
