use bevy::{
    ecs::{
        archetype::{ArchetypeId, Archetypes},
        component::{ComponentId, Components, StorageType},
        entity::Entities,
    },
    reflect::TypeRegistration,
};

pub fn list_resources(archetypes: &Archetypes, components: &Components) {
    let mut r: Vec<String> = archetypes
        .resource()
        .components()
        .map(|id| components.get_info(id).unwrap())
        // get_short_name removes the path information
        // i.e. `bevy_audio::audio::Audio` -> `Audio`
        // if you want to see the path info replace
        // `TypeRegistration::get_short_name` with `String::from`
        .map(|info| TypeRegistration::get_short_name(info.name()))
        .collect();

    // sort list alphebetically
    r.sort();
    r.iter().for_each(|name| println!("{}", name));
}

fn get_components_by_name(
    components: &Components,
    short: bool,
    filter: Option<&str>,
) -> Vec<(usize, String)> {
    let mut names = Vec::new();
    for id in 1..components.len() {
        if let Some(info) = components.get_info(ComponentId::new(id)) {
            if short {
                names.push((id, TypeRegistration::get_short_name(info.name())));
            } else {
                names.push((id, String::from(info.name())));
            }
        }
    }

    if let Some(filter) = filter {
        names
            .iter()
            .cloned()
            .filter(|(_, name)| name.contains(filter))
            .collect()
    } else {
        names
    }
}

pub fn list_components(c: &Components, short: bool, filter: Option<&str>) {
    let mut names = get_components_by_name(c, short, filter);

    names.sort();
    names
        .iter()
        .for_each(|(id, name)| println!("{} {}", id, name));
}

pub fn list_entities(e: &Entities) {
    println!("[entity index] [archetype id]");
    e.meta.iter().enumerate().for_each(|(id, meta)| {
        println!("{} {}", id, meta.location.archetype_id.index());
    });
}

pub fn list_archetypes(a: &Archetypes) {
    println!("[id] [entity count]");
    a.iter().for_each(|archetype| {
        println!(
            "{} {}",
            archetype.id().index(),
            archetype.entities().iter().count()
        )
    });
}

pub fn print_ecs_counts(a: &Archetypes, c: &Components, e: &Entities) {
    println!(
        "entities {}, components: {}, archetypes {}",
        e.len(),
        c.len(),
        a.len()
    );
}

pub fn find_archetypes_by_component_name(a: &Archetypes, c: &Components, component_name: &str) {
    let components = get_components_by_name(c, false, Some(component_name));

    if components.len() == 0 {
        println!("No component found with name {}", component_name);
        return;
    }

    if components.len() > 1 {
        println!("More than one component found with name {}", component_name);
        println!("Consider searching with '--componentid' instead");
        println!("");
        println!("[component id] [component name]");
        components
            .iter()
            .for_each(|(id, name)| println!("{} {}", id, name));
        return;
    }

    if let Some(id_name) = components.iter().next() {
        find_archetypes_by_component_id(a, id_name.0);
    };
}

pub fn find_archetypes_by_component_id(a: &Archetypes, component_id: usize) {
    let archetypes = a
        .iter()
        .filter(|archetype| archetype.components().any(|c| c.index() == component_id))
        .map(|archetype| archetype.id().index());

    println!("archetype ids:");
    archetypes.for_each(|id| print!("{}, ", id));
    println!();
}

pub fn find_archetypes_by_entity_id(a: &Archetypes, entity_id: u32) {
    let archetypes = a
        .iter()
        .filter(|archetype| archetype.entities().iter().any(|e| e.id() == entity_id))
        .map(|archetype| archetype.id().index());

    println!("archetype ids:");
    archetypes.for_each(|id| println!("{}", id));
}

pub fn find_entities_by_component_id(a: &Archetypes, component_id: usize) {
    let entities = a
        .iter()
        .filter(|archetype| archetype.components().any(|c| c.index() == component_id))
        .map(|archetype| archetype.entities())
        .flatten();

    entities.for_each(|id| println!("{}", id.id()));
}

pub fn print_archetype(a: &Archetypes, c: &Components, archetype_id: ArchetypeId) {
    if let Some(archetype) = a.get(archetype_id) {
        println!("id: {:?}", archetype.id());
        println!("table_id: {:?}", archetype.table_id());
        print!("entities ({}): ", archetype.entities().iter().count());
        archetype
            .entities()
            .iter()
            .for_each(|entity| print!("{}, ", entity.id()));
        println!("");
        // not sure what entity table rows is, so commenting out for now
        // print!(
        //     "entity table rows ({}): ",
        //     archetype.entity_table_rows().iter().count()
        // );
        // archetype
        //     .entity_table_rows()
        //     .iter()
        //     .for_each(|row| print!("{}, ", row));
        // println!("");
        print!(
            "table_components ({}): ",
            archetype.table_components().iter().count()
        );
        archetype
            .table_components()
            .iter()
            .map(|id| (id.index(), c.get_info(*id).unwrap()))
            .map(|(id, info)| (id, TypeRegistration::get_short_name(info.name())))
            .for_each(|(id, name)| print!("{} {}, ", id, name));
        println!("");

        print!(
            "sparse set components ({}): ",
            archetype.sparse_set_components().iter().count()
        );
        archetype
            .sparse_set_components()
            .iter()
            .map(|id| (id.index(), c.get_info(*id).unwrap()))
            .map(|(id, info)| (id, TypeRegistration::get_short_name(info.name())))
            .for_each(|(id, name)| print!("{} {}, ", id, name));
        println!("");
    } else {
        println!("No archetype found with id: {}", archetype_id.index());
    }
}

pub fn print_component(c: &Components, component_id: usize) {
    if let Some(info) = c.get_info(ComponentId::new(component_id)) {
        println!("Name: {}", info.name());
        println!("Id: {}", info.id().index());
        print!("StorageType: ");
        match info.storage_type() {
            StorageType::Table => println!("Table"),
            StorageType::SparseSet => println!("SparseSet"),
        }

        println!("SendAndSync: {}", info.is_send_and_sync());
    } else {
        println!("No component found with id: {}", component_id);
    }
}
