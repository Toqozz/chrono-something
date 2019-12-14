use amethyst::core::SystemDesc;
use amethyst::derive::SystemDesc;
use amethyst::core::math::base::{ Vector2 };
use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, ReadExpect, ReadStorage, System, SystemData, World, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};
use amethyst::window::ScreenDimensions;
use amethyst::renderer::camera::Camera;

use crate::states::world::Player;
use crate::sprite_animation::LayeredSpriteAnimation;

#[derive(SystemDesc)]
pub struct SpriteMouseDirectionSystem;

impl<'s> System<'s> for SpriteMouseDirectionSystem {
    type SystemData = (
        WriteStorage<'s, LayeredSpriteAnimation>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Player>,
        ReadStorage<'s, Camera>,
        Read<'s, InputHandler<StringBindings>>,
        ReadExpect<'s, ScreenDimensions>,
    );

    fn run(&mut self, (mut animations, transforms, players, cameras, input, screen_dimensions): Self::SystemData) {
        let (cam, cam_transform) = match (&cameras, &transforms).join().next() { Some(x) => x, _ => return };
        let (mouse_x, mouse_y) = match input.mouse_position() { Some(x) => x, _ => return };

        for (animation, transform, _) in (&mut animations, &transforms, &players).join() {
            let mouse_pos_world = cam.projection().screen_to_world_point(
                [mouse_x, mouse_y, 0.0].into(),
                screen_dimensions.diagonal(),
                cam_transform,
            );

            let translation= transform.translation().xy();
            let mouse: Vector2<f32> = [mouse_pos_world.x, mouse_pos_world.y].into();
            let dir = translation- mouse;

            let sprite_dir = find_compass_coordinate(dir);

            animation.current_row = sprite_dir;
        }
    }
}

fn find_compass_coordinate(dir: Vector2<f32>) -> usize {
    let coordinates = [
        Vector2::new(0., 1.),   // N.
        Vector2::new(1., 1.).normalize(),   // NE.
        Vector2::new(1., 0.),   // E.
        Vector2::new(1., -1.).normalize(),   // SE.
        Vector2::new(0., -1.),   // S.
        Vector2::new(-1., -1.).normalize(),   // SW.
        Vector2::new(-1., 0.),   // W.
        Vector2::new(-1., 1.).normalize(),   // NW.
    ];

    let idx = coordinates
        .iter()
        .enumerate()
        .max_by(
            |(_, x), (_, y)| {
                if x.dot(&dir) < y.dot(&dir) {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Less
                }
            });

    idx.unwrap().0
}

