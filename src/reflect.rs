use bevy::reflect::TypeRegistry;
use clap::{App, ArgMatches};

pub fn build_commands<'a>(app: App<'a>) -> App<'a> {
    let app = app.subcommand(App::new("reflect").about("get reflection info"));

    app
}

pub fn match_commands(matches: &ArgMatches, reflect: &TypeRegistry) -> String {
    match matches.subcommand() {
        Some(("reflect", _)) => list_reflection(reflect),
        _ => String::from(""),
    }
}

fn list_reflection(reflect: &TypeRegistry) -> String {
    let mut output = String::new();

    let type_registry = reflect.read();

    type_registry
        .iter()
        .for_each(|type_registration| output.push_str(&format!("{}\n", type_registration.short_name())));

    output
}
