use bevy::{prelude::*, utils::HashMap};
use bevy_fortress::prelude::*;
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg64;

use crate::{
    assets::AssetContext,
    constants::{MAP_HEIGHT, MAP_SIZE, MAP_WIDTH},
    types::{traits::SpawnAt, Rectangle, TileType},
};

#[derive(Component, Reflect, Default, Clone)]
pub struct Map {
    pub start_position: Coord,
    pub terrain: HashMap<Coord, Entity>,
}

impl Map {
    pub fn new(commands: &mut Commands, asset_context: &AssetContext) -> Option<Self> {
        let Some(floor_asset) = asset_context.get_terrain("floor") else {
            error!("Failed to find terrain: floor");
            return None;
        };

        let Some(wall_asset) = asset_context.get_terrain("wall") else {
            error!("Failed to find terrain: wall");
            return None;
        };
        let mut terrain = HashMap::new();

        let (rooms, map) = new_map_rooms_and_corridors();

        for coord in map.coord_iter() {
            let entity = match map.get_checked(coord) {
                TileType::Floor => {
                    let terrain_entity = floor_asset.spawn_at(commands, coord);
                    commands.entity(terrain_entity).insert(TileType::Floor);
                    terrain_entity
                },
                TileType::Wall => {
                    let terrain_entity = wall_asset.spawn_at(commands, coord);
                    commands.entity(terrain_entity).insert(TileType::Wall);
                    terrain_entity
                },
            };

            terrain.insert(coord, entity);
        }

        Some(Self {
            start_position: rooms[0].center(),
            terrain,
        })
    }

    pub fn get_entity_at(&self, position: Coord) -> Option<Entity> { self.terrain.get(&position).copied() }
}

fn new_map_rooms_and_corridors() -> (Vec<Rectangle>, Grid<TileType>) {
    let mut map = Grid::new_copy(MAP_SIZE, TileType::Wall);
    let mut rooms = Vec::new();

    const MAX_ROOMS: i32 = 30;
    const MIN_SIZE: u32 = 6;
    const MAX_SIZE: u32 = 10;

    let mut rng = Pcg64::from_entropy();

    'make_rooms: for _ in 0..MAX_ROOMS {
        let w = rng.gen_range(MIN_SIZE..=MAX_SIZE);
        let h = rng.gen_range(MIN_SIZE..=MAX_SIZE);
        let x = rng.gen_range(1..(MAP_WIDTH - w as i32 - 1));
        let y = rng.gen_range(1..(MAP_HEIGHT - h as i32 - 1));

        let new_room = Rectangle::new(Coord::new(x, y), Size::new(w, h));

        for other_room in rooms.iter() {
            if new_room.intersects(other_room) {
                continue 'make_rooms;
            }
        }

        apply_room_to_map(&new_room, &mut map);

        if !rooms.is_empty() {
            let new_center = new_room.center();
            let prev_center = rooms[rooms.len() - 1].center();
            if rng.gen_bool(0.5) {
                apply_horizontal_tunnel(&mut map, prev_center.x, new_center.x, prev_center.y);
                apply_vertical_tunnel(&mut map, new_center.x, prev_center.y, new_center.y);
            } else {
                apply_vertical_tunnel(&mut map, prev_center.x, prev_center.y, new_center.y);
                apply_horizontal_tunnel(&mut map, prev_center.x, new_center.x, new_center.y);
            }
        }

        rooms.push(new_room);
    }

    (rooms, map)
}

fn apply_room_to_map(room: &Rectangle, map: &mut Grid<TileType>) {
    for y in room.y_min()..=room.y_max() {
        for x in room.x_min()..=room.x_max() {
            let coord = Coord::new(x, y);
            if let Some(cell) = map.get_mut(coord) {
                *cell = TileType::Floor;
            }
        }
    }
}

fn apply_horizontal_tunnel(map: &mut Grid<TileType>, x1: i32, x2: i32, y: i32) {
    for x in x1.min(x2)..=x1.max(x2) {
        let coord = Coord::new(x, y);
        if let Some(cell) = map.get_mut(coord) {
            *cell = TileType::Floor;
        }
    }
}

fn apply_vertical_tunnel(map: &mut Grid<TileType>, x: i32, y1: i32, y2: i32) {
    for y in y1.min(y2)..=y1.max(y2) {
        let coord = Coord::new(x, y);
        if let Some(cell) = map.get_mut(coord) {
            *cell = TileType::Floor;
        }
    }
}
