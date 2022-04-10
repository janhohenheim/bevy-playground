use crate::bounds::Bounds2;
use crate::components::*;
use crate::resources::*;
use bevy::log;
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;

/// System to generate the complete Board
pub fn create_board(
    mut commands: Commands,
    board_options: Option<Res<BoardOptions>>,
    windows: Res<Windows>,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load("fonts/pixeled.ttf");
    let mine_image = asset_server.load("sprites/mine.png");

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

            spawn_tiles(
                parent,
                &tile_map,
                tile_size,
                options.tile_padding,
                Color::GRAY,
                mine_image,
                font,
            );
        });

    commands.insert_resource(Board {
        tile_map,
        bounds: Bounds2 {
            position: board_position.xy(),
            size: board_size,
        },
        tile_size,
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

fn spawn_tiles(
    parent: &mut ChildBuilder,
    tile_map: &TileMap,
    size: f32,
    padding: f32,
    color: Color,
    mine_image: Handle<Image>,
    font: Handle<Font>,
) {
    for (y, line) in tile_map.iter().enumerate() {
        for (x, tile) in line.iter().enumerate() {
            let mut cmd = parent.spawn();
            cmd.insert_bundle(SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::splat(size - padding as f32)),
                    ..Default::default()
                },
                transform: Transform::from_xyz(
                    (x as f32 * size) + (size / 2.),
                    (y as f32 * size) + (size / 2.),
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

            match tile {
                Tile::Mine => {
                    cmd.insert(Mine).with_children(|parent| {
                        parent.spawn_bundle(SpriteBundle {
                            sprite: Sprite {
                                custom_size: Some(Vec2::splat(size - padding)),
                                ..Default::default()
                            },
                            transform: Transform::from_xyz(0., 0., 1.),
                            texture: mine_image.clone(),
                            ..Default::default()
                        });
                    });
                }
                Tile::Neighbor(count) => {
                    cmd.insert(Neighbor { count: *count })
                        .with_children(|parent| {
                            parent.spawn_bundle(create_mine_count_text_bundle(
                                *count,
                                font.clone(),
                                size - padding,
                            ));
                        });
                }
                Tile::Empty => (),
            };
        }
    }
}

/// Generates the mine counter text 2D bundle for a given count
fn create_mine_count_text_bundle(count: u8, font: Handle<Font>, size: f32) -> Text2dBundle {
    let text = count.to_string();
    let color = match count {
        1 => Color::WHITE,
        2 => Color::GREEN,
        3 => Color::YELLOW,
        4 => Color::ORANGE,
        _ => Color::PURPLE,
    };
    Text2dBundle {
        text: Text {
            sections: vec![TextSection {
                value: text,
                style: TextStyle {
                    color,
                    font,
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
