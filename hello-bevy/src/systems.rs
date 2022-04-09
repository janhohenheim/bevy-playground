use crate::entities::{Name, *};
use bevy::prelude::*;

pub(crate) fn hello_world() {
    println!("Hello, world!");
}

pub(crate) fn add_people(mut commands: Commands) {
    commands
        .spawn()
        .insert(Person)
        .insert(Name("Jan".to_string()));

    commands
        .spawn()
        .insert(Person)
        .insert(Name("Alice".to_string()));

    commands
        .spawn()
        .insert(Person)
        .insert(Name("Bob".to_string()));
}

pub(crate) fn greet_people(query: Query<&Name, With<Person>>) {
    for name in query.iter() {
        println!("Hello, {}!", name.0);
    }
}
