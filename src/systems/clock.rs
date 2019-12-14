use amethyst::core::SystemDesc;
use amethyst::derive::SystemDesc;
use amethyst::core::Transform;
use amethyst::core::math::base::Vector4;
use amethyst::core::timing::Time;
use amethyst::ecs::{Join, Read, Write, ReadStorage, System, SystemData, World, WriteStorage};
//use amethyst::input::{ InputHandler, StringBindings };
use amethyst::renderer::palette::Srgba;
use amethyst::renderer::debug_drawing::DebugLines;

use crate::states::world::Clock;

#[derive(SystemDesc)]
pub struct ClockSystem;

impl<'s> System<'s> for ClockSystem {
    type SystemData = (
        WriteStorage<'s, Clock>,
        ReadStorage<'s, Transform>,
        Write<'s, DebugLines>,
        Read<'s, Time>,
    );
        //WriteStorage<'s, LayeredSpriteAnimation>,
        //WriteStorage<'s, SpriteRender>,

    fn run(&mut self, (mut clocks, transforms, mut debug_lines, time): Self::SystemData) {
        for (clock, transform) in (&mut clocks, &transforms).join() {
            let (mut clock_x, mut clock_y) = {
                let idx = clock.timer * ((clock.pos_data.len()-1) as f32);
                clock.pos_data[idx.floor() as usize]
            };

            clock_x -= 25.;
            clock_y = 34. - clock_y;

            // local translation.
            let matrix = transform.global_matrix();
            let pos = matrix * Vector4::new(0., 0., 0., 1.0);

            debug_lines.draw_line(
                [pos.x, pos.y, 0.0].into(),
                [pos.x + clock_x, pos.y + clock_y, 0.0].into(),
                Srgba::new(0., 1., 0., 1.),
            );

            clock.timer += time.delta_seconds() * clock.speed;
            while clock.timer > 1. {
                clock.timer -= 1.;
            }
        }
    }
}
