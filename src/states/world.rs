use amethyst::{
    assets::{ AssetStorage, Handle, Loader, },
    core::Parent,
    core::transform::Transform,
    window::ScreenDimensions,
    prelude::*,
    utils,
    renderer::{
        camera::Camera,
        Texture,
        SpriteSheet,
        formats::texture::ImageFormat,
        Transparent,
        SpriteRender,
        SpriteSheetFormat,
        debug_drawing::{ DebugLines },
    },
    ecs::{ NullStorage, Component, VecStorage },
};

use std::fs;
use std::path::Path;
use crate::tilemap::{ TileMap, Tile };
use crate::sprite_animation::LayeredSpriteAnimation;

#[derive(Default)]
pub struct Player;
impl Component for Player {
    type Storage = NullStorage<Self>;
}

pub struct Clock {
    pub timer: f32,
    pub speed: f32,
    pub pos_data: Vec<(f32, f32)>,
}
impl Component for Clock {
    type Storage = VecStorage<Self>;
}

pub struct WorldState;

impl SimpleState for WorldState {
    fn on_start(&mut self, mut data: StateData<'_, GameData<'_, '_>>) {
        self.init_level(&mut data.world);
        self.init_player(&mut data.world);
        self.init_camera(&mut data.world);

        // Debug.
        data.world.insert(DebugLines::new());
    }
}

impl WorldState {
    fn init_level(&mut self, world: &mut World) {
        let root = utils::application_root_dir().expect("couldn't get app root dir.");
        let resources = root.join("resources");

        // Needed because no systems use Tile.
        world.register::<Tile>();

        let tile_sheet =
            load_sprite_sheet_explicit(&resources.join("tilemap.png"), &resources.join("tiles.ron"), world);

        let mut map = TileMap::new(32.);
        map.load_level(&resources.join("level.data"));
        map.draw_level(world, tile_sheet);
        world.insert(map);
    }

    fn init_player(&mut self, world: &mut World) {
        let root = utils::application_root_dir().expect("couldn't get app root dir.");
        let resources = root.join("resources");

        // Init player.
        let player_sprite = load_sprite_sheet(&resources.join("player.png"), world);
        let player_transform = Transform::default();

        let player_render = SpriteRender {
            sprite_sheet: player_sprite,
            sprite_number: 0,
        };

        let walk = LayeredSpriteAnimation::new(
            1, 0, 4, 5, 0.15, false
        );

        let player = world
            .create_entity()
            .with(player_transform)
            .with(player_render)
            .with(walk)
            .with(Player)
            .with(Transparent)
            .with(crate::systems::draw_transforms::DebugTransform)
            .build();

        // Init clock.
        let clock_sprite = load_sprite_sheet(&resources.join("clock.png"), &world);
        let string = fs::read_to_string(&resources.join("clock.data")).expect("clock.data does not exist.");
        let clock_points = ron::de::from_str::<Vec<(f32, f32)>>(&string).expect("clock.data was malformed.");
        let clock = Clock {
            timer: 0.,
            speed: 0.2,
            pos_data: clock_points,
        };

        let parent = Parent::new(player);
        let mut clock_transform = Transform::default();
        clock_transform.set_translation_xyz(0., 80., 0.);

        let clock_render = SpriteRender {
            sprite_sheet: clock_sprite,
            sprite_number: 0,
        };

        world
            .create_entity()
            .with(clock_transform)
            .with(parent)
            .with(clock_render)
            .with(clock)
            .build();
    }

    fn init_camera(&mut self, world: &mut World) {
        let (width, height) = {
            let dimensions = world.read_resource::<ScreenDimensions>();
            (dimensions.width() / 2., dimensions.height() / 2.)
        };

        let mut transform = Transform::default();
        transform.set_translation_xyz(0., 0., 720.);

        let camera = Camera::standard_2d(width, height);

        world
            .create_entity()
            .with(transform)
            .with(camera)
            .build();
    }
}

fn load_texture(path: &Path, world: &World) -> Handle<Texture> {
    let loader = world.read_resource::<Loader>();
    loader.load(
        path.to_str().unwrap().to_owned(),
        ImageFormat::default(),
        (),
        &world.read_resource::<AssetStorage<Texture>>(),
    )
}

fn load_sprite_sheet_from_tex(path: &Path, tex: Handle<Texture>, world: &World) -> Handle<SpriteSheet> {
    let loader = world.read_resource::<Loader>();
    let sprite_sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();

    loader.load(
        path.to_str().unwrap().to_owned(),
        SpriteSheetFormat(tex),
        (),
        &sprite_sheet_storage,
    )
}

pub fn load_sprite_sheet_explicit(texture_path: &Path, ron_path: &Path, world: &World) -> Handle<SpriteSheet> {
    let tex_handle = load_texture(texture_path, world);
    let sprite_sheet_handle = load_sprite_sheet_from_tex(ron_path, tex_handle, world);

    sprite_sheet_handle
}

pub fn load_sprite_sheet(texture_path: &Path, world: &World) -> Handle<SpriteSheet> {
    // This should never happen.
    let ron_path = texture_path
        .clone()
        .parent()
        .expect("sprite file has no parent directory!")
        .join([texture_path.file_stem().unwrap().to_str().unwrap(), ".ron"].join(""));

    load_sprite_sheet_explicit(texture_path, &ron_path, world)
}
