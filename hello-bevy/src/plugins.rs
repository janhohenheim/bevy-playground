use crate::systems::*;
use bevy::prelude::*;

pub(crate) struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(add_people)
            .add_system(hello_world)
            .add_system(greet_people);
    }
}
