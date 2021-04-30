mod ecs;

use bevy::{
    ecs::{archetype::ArchetypeId, schedule::ShouldRun},
    prelude::*,
};
use clap::{App, AppSettings, ArgGroup, ArgMatches};
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

    match_commands(&matches, world);

    println!("");
}

fn build_app(app_name: &str) -> App {
    App::new(app_name)
        .subcommand(App::new("resume").about("resume running game"))
        .subcommand(App::new("quit").about("quit game"))
        .subcommand(
            App::new("counts").about("print counts of archetypes, components, and entities"),
        )
        .subcommand(
            App::new("archetypes")
                .about("get archetypes info")
                .alias("archetype")
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .subcommand(App::new("list")
                    .about("list all archetypes")
                )
                .subcommand(App::new("info")
                    .about("get info of one archetype")
                    .arg("--id [Id] 'id to get'")
                    .group(ArgGroup::new("search params")
                        .args(&["id"])
                        .required(true)
                    )
                )
                .subcommand(App::new("find")
                    .about("find a archetype")
                    .arg("--componentid   [ComponentId]   'find types that have components with ComponentId'")
                    .arg("--componentname [ComponentName] 'find types that have components with ComponentName'")
                    .arg("--entityid      [EntityId]      'find types that have entities with EntityId")
                    .group(ArgGroup::new("search params")
                        .args(&["componentid", "componentname", "entityid"])
                        .required(true)
                    )
                )
        )
        .subcommand(
            App::new("components")
                .about("get components info")
                .alias("component")
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .subcommand(App::new("list")
                    .about("list all components")
                    .arg("-f, --filter [Filter] 'filter list'")
                    .arg("-l, --long 'display long name'"),
                )
                .subcommand(App::new("info")
                    .about("get info of one component")
                    .arg("--id   [Id]   'id to get'")
                    .arg("--name [Name] 'name to get'")
                    .group(ArgGroup::new("search params")
                        .args(&["id", "name"])
                        .required(true)
                    )
                )
        )
        .subcommand(
            App::new("entities")
                .about("get entity info")
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .subcommand(
                    App::new("list")
                        .about("list all entities")
                )
                .subcommand(
                    App::new("find")
                        .about("find entity matching search params")
                        .arg("--componentid   [ComponentId]   'find types that have components with ComponentId'")
                        .arg("--componentname [ComponentName] 'find types that have components with ComponentName'")
                        .group(ArgGroup::new("search params")
                            .args(&["componentid", "componentname"])
                            .required(true)
                        )
                )
        )
        .subcommand(
            App::new("resources")
                .about("get resource info")
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .subcommand(
                    App::new("list")
                        .about("list all resources")
                )
        )
}

fn match_commands(matches: &ArgMatches, world: &mut World) {
    let a = world.archetypes();
    let c = world.components();
    let e = world.entities();
    
    match matches.subcommand() {
        Some(("resume", _)) => {
            let mut pause = world.get_resource_mut::<Pause>().unwrap();
            pause.0 = false;
            println!("...resuming game.")
        }
        Some(("quit", _)) => exit(0),
        Some(("archetypes", matches)) => match matches.subcommand() {
            Some(("list", _)) => ecs::list_archetypes(a),
            Some(("find", matches)) => {
                if let Ok(component_id) = matches.value_of_t("componentid") {
                    ecs::find_archetypes_by_component_id(a, component_id);
                }

                if let Some(component_name) = matches.value_of("componentname") {
                    ecs::find_archetypes_by_component_name(a, c, component_name);
                }

                if let Ok(entity_id) = matches.value_of_t("entityid") {
                    ecs::find_archetypes_by_entity_id(a, entity_id);
                }
            }
            Some(("info", matches)) => {
                if let Ok(id) = matches.value_of_t("id") {
                    ecs::print_archetype(a, c, ArchetypeId::new(id));
                }
            }
            _ => {}
        },
        Some(("components", matches)) => match matches.subcommand() {
            Some(("list", matches)) => {
                ecs::list_components(c, !matches.is_present("long"), matches.value_of("filter"))
            }
            Some(("info", matches)) => {
                if let Ok(id) = matches.value_of_t("id") {
                    ecs::print_component(c, id);
                }
            }
            _ => {}
        },
        Some(("entities", matches)) => match matches.subcommand() {
            Some(("list", _)) => ecs::list_entities(e),
            Some(("find", matches)) => {
                if let Ok(component_id) = matches.value_of_t("componentid") {
                    ecs::find_entities_by_component_id(a, component_id);
                }
            }
            _ => {}
        },
        Some(("resources", matches)) => match matches.subcommand() {
            Some(("list", _)) => ecs::list_resources(a, c),
            _ => {}
        },
        Some(("counts", _)) => ecs::print_ecs_counts(a, c, e),
        _ => {}
    }
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
