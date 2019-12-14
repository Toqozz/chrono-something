#![allow(dead_code)]

use amethyst::{
    core::transform::{ TransformBundle },
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
        plugins::RenderDebugLines,
    },
    utils::application_root_dir,
    input::{ InputBundle, StringBindings },
};

mod tilemap;
mod sprite_animation;
mod systems;
mod states;

use crate::states::WorldState;


fn main() -> amethyst::Result<()> {
    std::env::set_var("WINIT_HIDPI_FACTOR", "1.0");
    std::env::set_var("AMETHYST_LOG_LEVEL_FILTER", "warn");

    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let config_dir = app_root.join("config");
    let display_config_path = config_dir.join("display.ron");

    let input_bundle = InputBundle::<StringBindings>::new()
        .with_bindings_from_file("config/bindings.ron")?;

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)
                        .with_clear([0.1, 0.1, 0.1, 1.0]),
                )
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderDebugLines::default()),
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with(systems::PlayerMovementSystem, "player_movement_system", &["input_system"])
        .with(systems::SimpleSpriteAnimationSystem, "sprite_animation_system", &[])
        .with(systems::LayeredSpriteAnimationSystem, "layered_sprite_animation_system", &[])
        .with(systems::SpriteMouseDirectionSystem, "sprite_mouse_direction_system", &["input_system"])
        .with(systems::ClockSystem, "clock_system", &[]);

    let mut game = Application::new("./", WorldState, game_data)?;
    game.run();

    Ok(())
}
