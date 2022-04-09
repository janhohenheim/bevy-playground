mod entities;
mod plugins;
mod systems;
mod resources;

use bevy::prelude::*;
use plugins::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .run();
}
