use crate::components::*;
use crate::events::TileTriggerEvent;
use crate::resources::Board;
use bevy::log;
use bevy::prelude::*;

pub fn trigger_event_handler(
    mut commands: Commands,
    board: Res<Board>,
    mut tile_trigger_event_reader: EventReader<TileTriggerEvent>,
) {
    for entity in tile_trigger_event_reader
        .iter()
        .filter_map(|event| board.get_covered_tile(event.0))
    {
        commands.entity(entity).insert(Uncovered);
    }
}

pub fn uncover_tiles(
    mut commands: Commands,
    mut board: ResMut<Board>,
    // Entities in covered_tiles, which have the sprite component
    children: Query<(Entity, &Parent), With<Uncovered>>,
    parents: Query<(&Coordinates, Option<&Mine>, Option<&Neighbor>)>,
) {
    for (entity, parent) in children.iter() {
        // Remove tile cover
        commands.entity(entity).despawn_recursive();

        let (&coordinates, mine, neighbor) = match parents.get(parent.0) {
            Ok(parent) => parent,
            Err(error) => {
                log::error!("{}", error);
                continue;
            }
        };

        match board.uncover_tile(coordinates) {
            None => log::debug!(
                "Tried to uncover an already uncovered tile at {}",
                coordinates
            ),
            Some(_tile) => log::debug!("Uncovered tile at {} (entity: {:?})", coordinates, entity),
        }

        if mine.is_some() {
            log::info!("Boom!")
        } else if neighbor.is_none() {
            // Propagate event
            for entity in board.get_covered_neighbors(coordinates) {
                commands.entity(entity).insert(Uncovered);
            }
        };
    }
}
