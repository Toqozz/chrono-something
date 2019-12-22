use amethyst::core::SystemDesc;
use amethyst::derive::SystemDesc;
use amethyst::core::timing::Time;
use amethyst::ecs::{Join, Read, Write, ReadStorage, System, SystemData, World, WriteStorage};
use amethyst::input::{ InputHandler, StringBindings };

#[derive(SystemDesc)]
pub struct CheatSystem;

impl <'s> System<'s> for CheatSystem {
    type SystemData = (
        Write<'s, Time>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut time, input): Self::SystemData) {
        if input.action_is_down("timescale_faster").unwrap() {
            time.set_time_scale(4.);
        } else if input.action_is_down("timescale_slower").unwrap() {
            time.set_time_scale(0.25);
        } else {
            time.set_time_scale(1.);
        }
    }
}