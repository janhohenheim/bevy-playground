use crate::events::{TileMarkEvent, TileTriggerEvent};
use crate::resources::Board;
use bevy::input::{mouse::MouseButtonInput, ElementState};
use bevy::log;
use bevy::prelude::*;

pub fn input_handling(
    windows: Res<Windows>,
    board: Res<Board>,
    mut button_event_reader: EventReader<MouseButtonInput>,
    mut tile_trigger_event_writer: EventWriter<TileTriggerEvent>,
    mut tile_mark_event_writer: EventWriter<TileMarkEvent>,
) {
    let window = windows
        .get_primary()
        .expect("Error: No primary window found when trying to handle input");
    for (button, coordinates) in button_event_reader
        .iter()
        .filter(|event| event.state == ElementState::Pressed)
        .filter_map(|event| {
            let position = window.cursor_position()?;
            log::trace!("Mouse button pressed: {:?} at {}", event.button, position);
            let coordinates = board.convert_mouse_to_coordinates(position, window)?;
            Some((event.button, coordinates))
        })
    {
        match button {
            MouseButton::Left => {
                log::info!("Trying to uncover a tile at {}", coordinates);
                tile_trigger_event_writer.send(TileTriggerEvent(coordinates));
            }
            MouseButton::Right => {
                log::info!("Trying to mark a tile at {}", coordinates);
                tile_mark_event_writer.send(TileMarkEvent(coordinates));
            }
            _ => {}
        }
    }
}
