pub mod components;
pub mod resources;

use crate::components::Coordinates;
use bevy::log;
use bevy::prelude::*;
use resources::{tile_map::TileMap, BoardOptions, BoardPosition, TileSize};

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Self::create_board);
        log::info!("Loaded board plugin");
    }
}

impl BoardPlugin {
    /// System to generate the complete Board
    pub(crate) fn create_board(
        mut commands: Commands,
        board_options: Option<Res<BoardOptions>>,
        windows: Res<Windows>,
    ) {
        let options = match board_options {
            None => Default::default(),
            Some(options) => options.clone(),
        };
        let mut tile_map = TileMap::empty(options.map_size.0, options.map_size.1);
        tile_map.place_mines(options.mine_count);
        #[cfg(feature = "debug")]
        log::info!("{}", tile_map.console_output());

        let tile_size = match options.tile_size {
            TileSize::Fixed(size) => size,
            TileSize::Adaptive { min, max } => Self::calculate_adaptative_tile_size(
                windows,
                (min, max),
                (tile_map.width, tile_map.height),
            ),
        };

        let board_size = Vec2::new(
            tile_map.width as f32 * tile_size,
            tile_map.height as f32 * tile_size,
        );
        log::info!("Board size: {}", board_size);

        let board_position = match options.position {
            BoardPosition::Centered { offset } => {
                // Using the bottom left as anchor position
                Vec3::new(-(board_size.x / 2.), -(board_size.y / 2.), 0.) + offset
            }
            BoardPosition::Custom(position) => position,
        };

        commands
            .spawn()
            .insert(Name::new("Board"))
            .insert(Transform::from_translation(board_position))
            .insert(GlobalTransform::default())
            .with_children(|parent| {
                // We spawn the board background sprite at the center of the board, since the sprite pivot is centered
                parent
                    .spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            color: Color::WHITE,
                            custom_size: Some(board_size),
                            ..Default::default()
                        },
                        transform: Transform::from_xyz(board_size.x / 2., board_size.y / 2., 0.),
                        ..Default::default()
                    })
                    .insert(Name::new("Board Background"));

                for (y, line) in tile_map.iter().enumerate() {
                    for (x, _) in line.iter().enumerate() {
                        parent
                            .spawn_bundle(SpriteBundle {
                                sprite: Sprite {
                                    color: Color::GRAY,
                                    custom_size: Some(Vec2::splat(
                                        tile_size - options.tile_padding as f32,
                                    )),
                                    ..Default::default()
                                },
                                transform: Transform::from_xyz(
                                    (x as f32 * tile_size) + (tile_size / 2.),
                                    (y as f32 * tile_size) + (tile_size / 2.),
                                    // Closer to camera -> Drawn over background
                                    1.,
                                ),
                                ..Default::default()
                            })
                            .insert(Name::new(format!("Tile ({}, {})", x, y)))
                            .insert(Coordinates {
                                x: x as u16,
                                y: y as u16,
                            });
                    }
                }
            });
    }

    fn calculate_adaptative_tile_size(
        windows: Res<Windows>,
        (min, max): (f32, f32),
        (width, heigh): (u16, u16),
    ) -> f32 {
        let window = windows
            .get_primary()
            .expect("Error: No primary window found when generating board");
        let max_width = window.width() / width as f32;
        let max_height = window.height() / heigh as f32;
        max_width.min(max_height).clamp(min, max)
    }
}
