use amethyst::core::SystemDesc;
use amethyst::derive::SystemDesc;
use amethyst::core::Transform;
use amethyst::core::math::base::{ Vector2, Vector3 };
use amethyst::core::timing::Time;
use amethyst::ecs::{Join, Read, Write, ReadStorage, System, SystemData, World, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};
use amethyst::renderer::palette::Srgba;
use amethyst::renderer::debug_drawing::DebugLines;

use crate::states::world::Player;
use crate::tilemap::TileMap;
use crate::sprite_animation::{ LayeredSpriteAnimation};

#[derive(SystemDesc)]
pub struct PlayerMovementSystem;

impl<'s> System<'s> for PlayerMovementSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, LayeredSpriteAnimation>,
        ReadStorage<'s, Player>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
        Read<'s, TileMap>,
        Write<'s, DebugLines>,
    );

    fn run(&mut self, (mut transforms, mut animations, players, input, time, tilemap, mut debug_lines): Self::SystemData) {
        for (transform, animation, _) in (&mut transforms, &mut animations, &players).join() {
            debug_lines.draw_line(
                [transform.translation().x - 5., transform.translation().y, 0.0].into(),
                [transform.translation().x + 5., transform.translation().y, 0.0].into(),
                Srgba::new(0., 1., 0., 1.),
            );

            debug_lines.draw_line(
                [transform.translation().x, transform.translation().y - 5., 0.0].into(),
                [transform.translation().x, transform.translation().y + 5., 0.0].into(),
                Srgba::new(0., 1., 0., 1.),
            );

            let input = Vector2::new(input.axis_value("horizontal").unwrap(), input.axis_value("vertical").unwrap());
            if input.magnitude() <= 0. {
                animation.playing = false;
                animation.current_column = 0;
                return;
            }

            animation.playing = true;

            let move_dir = Vector3::new(input.x, input.y, 0.).normalize();

            if move_dir.x == 0. && move_dir.y > 0. {
                animation.current_row = 0;
            } else if move_dir.x > 0. && move_dir.y > 0. {
                animation.current_row = 1;
            } else if move_dir.x > 0. && move_dir.y < 0. {
                animation.current_row = 2;
            } else if move_dir.x == 0. && move_dir.y < 0. {
                animation.current_row = 3;
            } else if move_dir.x < 0. && move_dir.y < 0. {
                animation.current_row = 4;
            } else if move_dir.x < 0. && move_dir.y > 0. {
                animation.current_row = 5;
            } else {
                animation.current_row = 0;
            }

            //, `move_amount` should be high enough that we can move more than 1 pixel in a single
            // diagonal movement.  When we can't move a pixel's worth quickly enough, then movement
            // seems non-smooth.
            let move_amount = 100. * time.delta_seconds();

            let new_pos = tilemap.fix_movement(transform.translation().xy(), move_dir.xy(), move_amount);
            transform.set_translation(Vector3::new(new_pos.x, new_pos.y, 0.));
            //transform.prepend_translation(Vector3::new(movement.x, movement.y, 0.));
        }
    }
}