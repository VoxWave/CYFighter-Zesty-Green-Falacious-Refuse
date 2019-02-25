use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Component, DenseVecStorage};
use amethyst::prelude::*;
use amethyst::renderer::{
    Camera, PngFormat, Projection, SpriteRender, SpriteSheet,
    SpriteSheetFormat, SpriteSheetHandle, Texture, TextureMetadata,
};

use crate::fight_stick::{Stick, StickState, Button, ButtonType};

pub const VIEW_HEIGHT: f32 = 100.0;
pub const VIEW_WIDTH: f32 = 200.0;

fn initialize_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_z(1.);
    world.create_entity()
    .with(
        Camera::from(
            Projection::orthographic(
                0., 
                VIEW_WIDTH, 
                0., 
                VIEW_HEIGHT,
            )
        )
    )
    .with(transform)
    .build();
}

fn initialize_stick_and_buttons(world: &mut World, sprite_sheet: SpriteSheetHandle) {
    let mut stick_transform = Transform::default();
    let mut a_transform = Transform::default();
    let mut b_transform = Transform::default();
    let mut c_transform = Transform::default();
    let mut d_transform = Transform::default();
    let mut e_transform = Transform::default();

    let y = VIEW_HEIGHT/2.;
    stick_transform.set_xyz(VIEW_WIDTH/4., y, 0.);
    stick_transform.set_scale(0.1, 0.1, 0.1);

    a_transform.set_xyz(VIEW_WIDTH/2. + VIEW_WIDTH/10. * 0., y, 0.);
    a_transform.set_scale(0.1, 0.1, 0.1);
    b_transform.set_xyz(VIEW_WIDTH/2. + VIEW_WIDTH/10. * 1., y, 0.);
    b_transform.set_scale(0.1, 0.1, 0.1);
    c_transform.set_xyz(VIEW_WIDTH/2. + VIEW_WIDTH/10. * 2., y, 0.);
    c_transform.set_scale(0.1, 0.1, 0.1);
    d_transform.set_xyz(VIEW_WIDTH/2. + VIEW_WIDTH/10. * 3., y, 0.);
    d_transform.set_scale(0.1, 0.1, 0.1);
    e_transform.set_xyz(VIEW_WIDTH/2. + VIEW_WIDTH/10. * 4., y, 0.);
    e_transform.set_scale(0.1, 0.1, 0.1);

    let stick_render = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 0,
    };

    create_stick_entity(world, StickState::Neutral, stick_transform, stick_render.clone());

    let button_render = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 0,
    };

    create_button_entity(world, ButtonType::A, a_transform, button_render.clone());
    create_button_entity(world, ButtonType::B, b_transform, button_render.clone());
    create_button_entity(world, ButtonType::C, c_transform, button_render.clone());
    create_button_entity(world, ButtonType::D, d_transform, button_render.clone());
    create_button_entity(world, ButtonType::E, e_transform, button_render.clone());
}

fn create_button_entity(world: &mut World, button_type: ButtonType, transform: Transform, sprite: SpriteRender) {
    world
        .create_entity()
        .with(sprite)
        .with(Button(button_type, false))
        .with(transform)
        .build();
}

fn create_stick_entity(world: &mut World, state: StickState, transform: Transform, sprite: SpriteRender) {
    world
        .create_entity()
        .with(Stick(state))
        .with(sprite)
        .with(transform)
        .build();
}

fn load_sprite_sheet(world: &mut World) -> SpriteSheetHandle {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "assets/textures/stickball.png",
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &texture_storage,
        )
    };
    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "assets/textures/stickball_spritesheet.ron", // Here we load the associated ron file
        SpriteSheetFormat,
        texture_handle, // We pass it the handle of the texture we want it to use
        (),
        &sprite_sheet_store,
    )
}

pub struct Game;

impl SimpleState for Game {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.register::<Button>();
        world.register::<Stick>();
        initialize_camera(world);
        let stick_sprite_handle = load_sprite_sheet(world);
        initialize_stick_and_buttons(world, stick_sprite_handle); 
    }
}
