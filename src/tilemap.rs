use amethyst::{
    assets::{ Handle },
    core::{
        transform::Transform,
        math::base::Vector2,
    },
    ecs::{ Component, VecStorage },
    renderer::{ SpriteSheet, SpriteRender, Transparent },
    prelude::*,
};

use ron;
use std::path::Path;

pub struct Tile {
    pub walkable: bool,
}

impl Component for Tile {
    type Storage = VecStorage<Self>;
}

#[derive(Default)]
pub struct TileMap {
    pub level: Vec<Vec<usize>>,

    pub tile_size: f32,
}

impl TileMap {
    pub fn new(tile_size: f32) -> Self {
        Self {
            level: Vec::new(),
            tile_size,
        }
    }

    pub fn load_level(&mut self, path: &Path) {
        let string = std::fs::read_to_string(path).unwrap();
        let level = ron::de::from_str::<Vec<Vec<usize>>>(&string).unwrap();
        self.level = level;
    }

    pub fn draw_level(&self, world: &mut World, handle: Handle<SpriteSheet>) {
        for i in (0..self.level.len()).rev() {
            for j in (0..self.level[i].len()).rev() {
                let mut pos = Vector2::new(
                    j as f32 * self.tile_size,
                    i as f32 * self.tile_size,
                );

                pos = convert_2d_to_iso(pos);

                self.place_sprite(world, pos, self.level[i][j], handle.clone());
            }
        }
    }

    fn place_sprite(&self, world: &mut World, pos: Vector2<f32>, sprite_idx: usize, handle: Handle<SpriteSheet>) {
        let mut t = Transform::default();
        t.set_translation_xyz(pos.x, pos.y, -pos.y);

        let render = SpriteRender {
            sprite_sheet: handle,
            sprite_number: sprite_idx,
        };

        let walkable = {
            if sprite_idx == 1 {
                false
            } else {
                true
            }
        };

        let tile = Tile {
            walkable,
        };

        world
            .create_entity()
            .with(t)
            .with(render)
            .with(tile)
            .with(Transparent)
            .build();
    }

    pub fn fix_movement(&self, pos: Vector2<f32>, move_dir: Vector2<f32>, move_amount: f32) -> Vector2<f32> {
        let move_dir_2d = convert_iso_to_2d(move_dir);
        let mut move_2d = move_dir_2d * move_amount;

        let pos_2d = convert_iso_to_2d(pos);
        let can_move_x = self.query_walkable(pos_2d + Vector2::new(move_2d.x, 0.));
        let can_move_y = self.query_walkable(pos_2d + Vector2::new(0., move_2d.y));

        if !can_move_x {
            move_2d.x = 0.;
        }

        if !can_move_y {
            move_2d.y = 0.;
        }

        let mut new_pos = pos_2d + move_2d;
        if new_pos.x % 32. < 1. && !can_move_x {
            new_pos.x = new_pos.x.floor() + 0.001;
        } else if new_pos.x % 32. > 31. && !can_move_x {
            new_pos.x = new_pos.x.ceil() - 0.001;
        }

        if new_pos.y % 32. < 1. && !can_move_y {
            new_pos.y = new_pos.y.floor() + 0.001;
        } else if new_pos.y % 32. > 31. && !can_move_y {
            new_pos.y = new_pos.y.ceil() - 0.001;
        }

        convert_2d_to_iso(new_pos)
    }

    fn query_walkable(&self, pos: Vector2<f32>) -> bool {
        let pos_grid = pos / self.tile_size;
        let (x, y) = (pos_grid.x.ceil() as usize, pos_grid.y.ceil() as usize);
        if y >= self.level.len() || x >= self.level[y].len() {
            return false;
        }

        let tile = self.level[y][x];
        tile == 1
    }
}

pub fn convert_iso_to_2d(point: Vector2<f32>) -> Vector2<f32> {
    let x = (2. * point.y + point.x) / 2.;
    let y = (2. * point.y - point.x) / 2.;
    Vector2::new(x, y)
}

pub fn convert_2d_to_iso(point: Vector2<f32>) -> Vector2<f32> {
    let x = point.x - point.y;
    let y = (point.x + point.y) / 2.;
    Vector2::new(x, y)
}
