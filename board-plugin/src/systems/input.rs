use crate::resources::Board;
use bevy::input::{mouse::MouseButtonInput, ElementState};
use bevy::log;
use bevy::prelude::*;

pub fn input_handling(
    windows: Res<Windows>,
    board: Res<Board>,
    mut button_event_reader: EventReader<MouseButtonInput>,
) {
    let window = windows
        .get_primary()
        .expect("Error: No primary window found when trying to handle input");
    for (button, coordinates) in button_event_reader
        .iter()
        .filter(|event| event.state == ElementState::Pressed)
        .filter_map(|event| window.cursor_position().map(|position| (event, position)))
        .map(|(event, position)| {
            log::trace!("Mouse button pressed: {:?} at {}", event.button, position);
            (event.button, position)
        })
        .filter_map(|(button, position)| {
            board
                .convert_mouse_to_coordinates(position, &window)
                .map(|coordinates| (button, coordinates))
        })
    {
        match button {
            MouseButton::Left => {
                log::info!("Trying to uncover a tile at {}", coordinates);
            }
            MouseButton::Right => {
                log::info!("Trying to mark a tile at {}", coordinates);
            }
            _ => {}
        }
    }
}
