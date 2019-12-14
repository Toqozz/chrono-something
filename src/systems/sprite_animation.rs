use amethyst::core::SystemDesc;
use amethyst::derive::SystemDesc;
use amethyst::renderer::SpriteRender;
use amethyst::core::timing::Time;
use amethyst::ecs::{Join, Read, System, SystemData, World, WriteStorage};

use crate::sprite_animation::SimpleSpriteAnimation;
use crate::sprite_animation::LayeredSpriteAnimation;

#[derive(SystemDesc)]
pub struct SimpleSpriteAnimationSystem;

impl<'s> System<'s> for SimpleSpriteAnimationSystem {
    type SystemData = (
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, SimpleSpriteAnimation>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut sprite_renders, mut animations, time): Self::SystemData) {
        for (sprite_render, animation) in (&mut sprite_renders, &mut animations).join() {
            if !animation.playing {
                continue;
            }

            animation.elapsed_time += time.delta_seconds();
            let frame_count = (animation.elapsed_time / animation.time_per_frame) as usize
                                       % animation.frames;

            if frame_count != animation.current_frame {
                animation.current_frame = frame_count;
                sprite_render.sprite_number = animation.start_frame_idx + frame_count;
            }
        }
    }
}



#[derive(SystemDesc)]
pub struct LayeredSpriteAnimationSystem;

impl<'s> System<'s> for LayeredSpriteAnimationSystem {
    type SystemData = (
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, LayeredSpriteAnimation>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut sprite_renders, mut animations, time): Self::SystemData) {
        for (sprite_render, animation) in (&mut sprite_renders, &mut animations).join() {
            if !animation.playing {
                animation.current_column = 0;
                sprite_render.sprite_number = animation.current_column + (animation.current_row * animation.sheet_columns);
                continue;
            }

            animation.elapsed_time += time.delta_seconds();
            let column = animation.start_column + ((animation.elapsed_time / animation.time_per_frame) as usize
                % (animation.animation_columns));

            if column != animation.current_column {
                animation.current_column = column;
            }

            sprite_render.sprite_number = animation.current_column + (animation.current_row * animation.sheet_columns);
        }
    }
}
