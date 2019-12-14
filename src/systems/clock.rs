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
            let clock_pos = {
                let idx = clock.timer * ((clock.pos_data.len()-1) as f32);
                clock.pos_data[idx.floor() as usize]
            };

            // local translation.
            let t = transform.translation();

            let matrix = transform.global_matrix();
            let pos =  matrix * Vector4::new(t.x, t.y, t.z, 1.0);



            debug_lines.draw_line(
                [pos.x, pos.y, 0.0].into(),
                [pos.x + clock_pos.0, pos.y + clock_pos.1, 0.0].into(),
                Srgba::new(0., 1., 0., 1.),
            );

            clock.timer += time.delta_seconds();
            if clock.timer > 1. {
                clock.timer -= 1.;
            }
        }
    }
}
