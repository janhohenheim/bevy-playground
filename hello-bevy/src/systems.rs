use crate::entities::{Name, *};
use crate::resources::*;
use bevy::prelude::*;

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

pub(crate) fn greet_people(
    time: Res<Time>,
    mut timer: ResMut<GreetTimer>,
    query: Query<&Name, With<Person>>,
) {
    // Update our timer with the time elapsed since the last update
    // if that caused the timer to finish, we can greet everyone
    if timer.0.tick(time.delta()).just_finished() {
        for name in query.iter() {
            println!("Hello, {}!", name.0);
        }
    }
}
