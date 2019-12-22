use amethyst::core::SystemDesc;
use amethyst::derive::SystemDesc;
use amethyst::core::Transform;
use amethyst::core::math::base::{ Vector2, Vector3 };
use amethyst::core::timing::Time;
use amethyst::ecs::{ Join, Read, Write, ReadStorage, System, SystemData, World, NullStorage, Component };
use amethyst::renderer::palette::Srgba;
use amethyst::renderer::debug_drawing::DebugLines;

#[derive(Default)]
pub struct DebugTransform;
impl Component for DebugTransform {
    type Storage = NullStorage<Self>;
}

#[derive(SystemDesc)]
pub struct DrawTransformsSystem;

impl<'s> System<'s> for DrawTransformsSystem {
    type SystemData = (
        ReadStorage<'s, Transform>,
        ReadStorage<'s, DebugTransform>,
        Write<'s, DebugLines>,
    );

    fn run(&mut self, (transforms, debug_transforms, mut debug_lines): Self::SystemData) {
        for (transform, _) in (&transforms, &debug_transforms).join() {
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
        }
    }
}
