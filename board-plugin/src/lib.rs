pub mod assets;
mod bounds;
pub mod components;
mod events;
pub mod resources;
mod systems;

use bevy::ecs::schedule::StateData;
use bevy::log;
use bevy::prelude::*;
#[cfg(feature = "debug")]
use bevy_inspector_egui::RegisterInspectable;
#[cfg(feature = "debug")]
use components::*;
use resources::Board;

pub struct BoardPlugin<T> {
    pub current_state: T,
}

impl<T: StateData> Plugin for BoardPlugin<T> {
    fn build(&self, app: &mut App) {
        let initial_state = self.current_state.clone();
        app.add_system_set(
            // Active when the initial state is pushed onto the stack
            SystemSet::on_enter(initial_state.clone()).with_system(systems::startup::create_board),
        )
        .add_system_set(
            // Active when the initial state active, i.e. on top of the stack
            SystemSet::on_update(initial_state.clone())
                .with_system(systems::input_handling)
                .with_system(systems::uncover::trigger_event_handler),
        )
        .add_system_set(
            // Active when the initial state is in the stack, no matter where
            SystemSet::on_in_stack_update(initial_state.clone())
                .with_system(systems::uncover::uncover_tiles),
        )
        .add_system_set(
            // Active when the initial state is popped off the stack
            SystemSet::on_exit(initial_state).with_system(Self::cleanup_board),
        )
        .add_event::<events::TileTriggerEvent>();
        log::info!("Loaded board plugin");
        #[cfg(feature = "debug")]
        {
            app.register_inspectable::<Coordinates>()
                .register_inspectable::<Neighbor>()
                .register_inspectable::<Mine>()
                .register_inspectable::<Uncovered>();
        }
    }
}

impl<T> BoardPlugin<T> {
    fn cleanup_board(mut commands: Commands, board: Res<Board>) {
        commands.entity(board.entity).despawn_recursive();
        commands.remove_resource::<Board>();
    }
}
