use crate::assets::board::BoardAssets;
use crate::bounds::Bounds2;
use crate::components::*;
use crate::resources::*;
use bevy::log;
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy::utils::AHashExt;
use bevy::utils::HashMap;

/// System to generate the complete Board
pub fn create_board(
    mut commands: Commands,
    board_options: Option<Res<BoardOptions>>,
    windows: Res<Windows>,
    board_assets: Res<BoardAssets>,
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
        TileSize::Adaptive { min, max } => {
            calculate_adaptative_tile_size(windows, (min, max), (tile_map.width, tile_map.height))
        }
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

    let mut covered_tiles = HashMap::with_capacity((tile_map.width * tile_map.height).into());
    let mut safe_start_entity = None;

    let board_entity = commands
        .spawn()
        .insert(Name::new("Board"))
        .insert(Transform::from_translation(board_position))
        .insert(GlobalTransform::default())
        .with_children(|parent| {
            // We spawn the board background sprite at the center of the board, since the sprite pivot is centered
            parent
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: board_assets.board_material.color,
                        custom_size: Some(board_size),
                        ..Default::default()
                    },
                    texture: board_assets.board_material.texture.clone(),
                    transform: Transform::from_xyz(board_size.x / 2., board_size.y / 2., 0.),
                    ..Default::default()
                })
                .insert(Name::new("Board Background"));

            let graphic_options = TileGraphicOptions {
                size: tile_size,
                padding: options.tile_padding,
            };
            spawn_tiles(
                parent,
                &tile_map,
                &board_assets,
                &graphic_options,
                &mut covered_tiles,
                &mut safe_start_entity,
            );
        })
        .id();

    commands.insert_resource(Board {
        tile_map,
        bounds: Bounds2 {
            position: board_position.xy(),
            size: board_size,
        },
        tile_size,
        covered_tiles,
        entity: board_entity,
    });

    if options.safe_start {
        if let Some(entity) = safe_start_entity {
            commands.entity(entity).insert(Uncovered);
        }
    }
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

struct TileGraphicOptions {
    size: f32,
    padding: f32,
}
impl TileGraphicOptions {
    fn effective_sprite_size(&self) -> f32 {
        self.size - self.padding
    }
}

fn spawn_tiles(
    parent: &mut ChildBuilder,
    tile_map: &TileMap,
    assets: &BoardAssets,
    graphic_options: &TileGraphicOptions,
    covered_tiles: &mut HashMap<Coordinates, Entity>,
    safe_start_entity: &mut Option<Entity>,
) {
    for (y, line) in tile_map.iter().enumerate() {
        for (x, tile) in line.iter().enumerate() {
            let coordinates = Coordinates {
                x: x as u16,
                y: y as u16,
            };

            let mut cmd = parent.spawn();
            cmd.insert_bundle(SpriteBundle {
                sprite: Sprite {
                    color: assets.tile_material.color,
                    custom_size: Some(Vec2::splat(graphic_options.effective_sprite_size())),
                    ..Default::default()
                },
                transform: Transform::from_xyz(
                    (x as f32 * graphic_options.size) + (graphic_options.size / 2.),
                    (y as f32 * graphic_options.size) + (graphic_options.size / 2.),
                    // Closer to camera -> Drawn over background
                    1.,
                ),
                texture: assets.tile_material.texture.clone(),
                ..Default::default()
            })
            .insert(Name::new(format!("Tile ({}, {})", x, y)))
            .insert(coordinates);

            cmd.with_children(|parent| {
                let entity = parent
                    .spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(Vec2::splat(graphic_options.effective_sprite_size())),
                            color: assets.covered_tile_material.color,
                            ..Default::default()
                        },
                        transform: Transform::from_xyz(0., 0., 2.),
                        texture: assets.covered_tile_material.texture.clone(),
                        ..Default::default()
                    })
                    .insert(Name::new("Tile Cover"))
                    .id();
                covered_tiles.insert(coordinates, entity);
                if safe_start_entity.is_none() && *tile == Tile::Empty {
                    *safe_start_entity = Some(entity);
                }
            });

            match tile {
                Tile::Mine => {
                    cmd.insert(Mine).with_children(|parent| {
                        parent.spawn_bundle(SpriteBundle {
                            sprite: Sprite {
                                custom_size: Some(Vec2::splat(
                                    graphic_options.effective_sprite_size(),
                                )),
                                color: assets.mine_material.color,
                                ..Default::default()
                            },
                            transform: Transform::from_xyz(0., 0., 1.),
                            texture: assets.mine_material.texture.clone(),
                            ..Default::default()
                        });
                    });
                }
                Tile::Neighbor(count) => {
                    cmd.insert(Neighbor { count: *count })
                        .with_children(|parent| {
                            parent.spawn_bundle(create_mine_count_text_bundle(
                                *count,
                                assets,
                                graphic_options.effective_sprite_size(),
                            ));
                        });
                }
                Tile::Empty => (),
            };
        }
    }
}

/// Generates the mine counter text 2D bundle for a given count
fn create_mine_count_text_bundle(count: u8, board_assets: &BoardAssets, size: f32) -> Text2dBundle {
    let text = count.to_string();
    let color = board_assets.get_mine_counter_color(count);
    Text2dBundle {
        text: Text {
            sections: vec![TextSection {
                value: text,
                style: TextStyle {
                    color,
                    font: board_assets.neighbor_font.clone(),
                    font_size: size,
                },
            }],
            alignment: TextAlignment {
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Center,
            },
        },
        // Z = 1 -> Closer to camera -> Drawn over background
        transform: Transform::from_xyz(0., 0., 1.),
        ..Default::default()
    }
}
