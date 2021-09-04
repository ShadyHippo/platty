use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::{transform::Transform, Time},
    ecs::{Component, DenseVecStorage, Entity},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    ui::{Anchor, LineMode, TtfFormat, UiText, UiTransform},
};

pub const HEIGHT: f32 = 100.;
pub const WIDTH: f32 = 100.;
pub const PLAYER_HEIGHT: f32 = 5.;
pub const PLAYER_WIDTH: f32 = 5.;

#[derive(Default)]
pub struct Platty {
    sprite_sheet_handle: Option<Handle<SpriteSheet>>,
}

impl SimpleState for Platty {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        self.sprite_sheet_handle.replace(load_sprite_sheet(world));
        initialize_camera(world);
        initialize_player(world, self.sprite_sheet_handle.clone().unwrap(), 50., 50.);
    }
}

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/player_cute.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/player_cute.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}

fn initialize_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(WIDTH * 0.5, HEIGHT * 0.5, 1.);

    world
        .create_entity()
        .with(Camera::standard_2d(WIDTH, HEIGHT))
        .with(transform)
        .build();
}

fn initialize_player(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>, x: f32, y: f32) {
    let sprite_render = SpriteRender::new(sprite_sheet_handle, 0);
}
