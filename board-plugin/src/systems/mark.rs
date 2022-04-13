use crate::assets::board::BoardAssets;
use crate::events::TileMarkEvent;
use crate::resources::Board;
use bevy::log;
use bevy::prelude::*;

pub fn mark_tiles(
    mut commands: Commands,
    mut board: ResMut<Board>,
    board_assets: Res<BoardAssets>,
    mut tile_mark_event_reader: EventReader<TileMarkEvent>,
    query: Query<&Children>,
) {
    let tile_size = board.tile_size;
    for (entity, is_marked_now) in tile_mark_event_reader
        .iter()
        .filter_map(|event| board.toggle_mark(event.0))
    {
        if is_marked_now {
            commands.entity(entity).with_children(|child_builder| {
                child_builder.spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: board_assets.flag_material.color,
                        custom_size: Some(Vec2::splat(tile_size)),
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(0., 0., 1.),
                    texture: board_assets.flag_material.texture.clone(),
                    ..Default::default()
                });
            });
        } else {
            // Note: This query could be optimized with a new TileCover component,
            // therefore avoiding querying every entity with children

            let children = match query.get(entity) {
                Ok(children) => children,
                Err(err) => {
                    log::error!(
                        "Tried to remove tile mark from entity without children: {}",
                        err
                    );
                    continue;
                }
            };
            for &child in children.iter() {
                commands.entity(child).despawn_recursive();
            }
        }
    }
}
