use bevy::reflect::TypeRegistry;
use clap::{App, ArgMatches, AppSettings};

pub fn build_commands<'a>(app: App<'a>) -> App<'a> {
    let app = app.subcommand(
        App::new("reflect")
            .about("get reflection info")
            .setting(AppSettings::SubcommandRequiredElseHelp)
            .subcommand(App::new("list").about("list all reflection types")));

    app
}

pub fn match_commands(matches: &ArgMatches, reflect: &TypeRegistry) -> String {
    match matches.subcommand() {
        Some(("reflect", matches)) => match matches.subcommand() {
            Some(("list", _)) => list_reflection(reflect),
            _ => String::from("this line should not be able to be run"),
        },
        _ => String::from("this line should not be able to be run"),
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
