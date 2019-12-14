use amethyst::ecs::{ Component, VecStorage };

#[derive(Default)]
pub struct SimpleSpriteAnimation {
    pub start_frame_idx: usize,
    pub frames: usize,
    pub current_frame: usize,
    pub time_per_frame: f32,
    pub elapsed_time: f32,
    pub playing: bool,
}

impl Component for SimpleSpriteAnimation {
    type Storage = VecStorage<Self>;
}


impl SimpleSpriteAnimation {
    pub fn new(start_frame_idx: usize, frames: usize, time_per_frame: f32, play: bool) -> Self {
        Self {
            start_frame_idx,
            frames,
            current_frame: 0,
            time_per_frame,
            elapsed_time: 0.,
            playing: play,
        }
    }
}

#[derive(Default)]
pub struct LayeredSpriteAnimation {
    pub start_column: usize,
    pub animation_columns: usize,
    pub sheet_columns: usize,
    pub current_column: usize,
    pub current_row: usize,
    pub time_per_frame: f32,
    pub elapsed_time: f32,
    pub playing: bool,
}

impl Component for LayeredSpriteAnimation {
    type Storage = VecStorage<Self>;
}

impl LayeredSpriteAnimation {
    pub fn new(start_column: usize, start_row: usize, animation_columns: usize, sheet_columns: usize, time_per_frame: f32, play: bool) -> Self {
        Self {
            start_column,
            animation_columns,
            sheet_columns,
            current_column: start_column,
            current_row: start_row,
            time_per_frame,
            elapsed_time: 0.,
            playing: play,
        }
    }
}
