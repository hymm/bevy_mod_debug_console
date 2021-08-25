use crate::app::{build_commands, input_pause, match_commands, pause, EnteringConsole, Pause};
use bevy::{
    ecs::{archetype::Archetypes, component::Components, entity::Entities},
    prelude::*,
    reflect::TypeRegistry,
    tasks::AsyncComputeTaskPool,
};
use crossbeam::channel::{bounded, Receiver};
use std::io::{self, BufRead, Write};

fn parse_input(
    a: &Archetypes,
    c: &Components,
    e: &Entities,
    reflect: Res<TypeRegistry>,
    mut pause: ResMut<Pause>,
    line_channel: Res<Receiver<String>>,
) {
    if let Ok(line) = line_channel.try_recv() {
        let app_name = "";
        println!("");
        let split = line.split_whitespace();
        let mut args = vec![app_name];
        args.append(&mut split.collect());

        let matches_result = build_commands(app_name).try_get_matches_from(args);

        if let Err(e) = matches_result {
            println!("{}", e.to_string());
            print!(">>> ");
            io::stdout().flush().unwrap();
            return;
        }

        let matches = matches_result.unwrap();

        let output = match_commands(&matches, a, c, e, &mut pause, &*reflect);

        println!("{}", output);
        print!(">>> ");
        io::stdout().flush().unwrap();
    }
}

fn spawn_io_thread(mut commands: Commands, thread_pool: Res<AsyncComputeTaskPool>) {
    println!("Bevy Console Debugger.  Type 'help' for list of commands.");
    print!(">>> ");
    io::stdout().flush().unwrap();

    let (tx, rx) = bounded(1);
    let task = thread_pool.spawn(async move {
        let stdin = io::stdin();
        loop {
            let line = stdin.lock().lines().next().unwrap().unwrap();
            tx.send(line)
                .expect("error sending user input to other thread");
        }
    });
    task.detach();
    commands.insert_resource(rx);
}

pub struct ConsoleDebugPlugin;
impl Plugin for ConsoleDebugPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Pause(false))
            .insert_resource(EnteringConsole(false))
            .add_startup_system(spawn_io_thread.system())
            .add_system(parse_input.system().with_run_criteria(pause.system()))
            .add_system(input_pause.system());
    }
}
