use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Component, DenseVecStorage};
use amethyst::prelude::*;
use amethyst::renderer::{
    Camera, PngFormat, Projection, SpriteRender, SpriteSheet,
    SpriteSheetFormat, SpriteSheetHandle, Texture, TextureMetadata,
};

use crate::fight_stick

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

fn initialize_stick_and_buttons(world: &mut World) {
    let mut stick_transform = Transform::default();
    let mut a_transform = Transform::default();
    let mut b_transform = Transform::default();
    let mut c_transform = Transform::default();
    let mut d_transform = Transform::default();
    let mut e_transform = Transform::default();

    let y = VIEW_HEIGHT/2.;
    stick_transform.set_xyz(VIEW_WIDTH/4., y, 0.);
    a_transform.set_xyz(VIEW_WIDTH/2. + VIEW_WIDTH/10.*1., y, 0.);
    b_transform.set_xyz(VIEW_WIDTH/2. + VIEW_WIDTH/10.*2., y, 0.);
    c_transform.set_xyz(VIEW_WIDTH/2. + VIEW_WIDTH/10.*3., y, 0.);
    d_transform.set_xyz(VIEW_WIDTH/2. + VIEW_WIDTH/10.*4., y, 0.);
    e_transform.set_xyz(VIEW_WIDTH/2. + VIEW_WIDTH/10.*5., y, 0.);

    world
        .create_entity()
        .with(Stick(StickState::Neutral))
}

pub struct Game;

impl SimpleState for Game {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        initialize_camera(world);
    }
}
