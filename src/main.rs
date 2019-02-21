#[macro_use]
extern crate specs_derive;

use nalgebra as na;

type Point = na::Point2<f64>;

use amethyst::{
    core::transform::TransformBundle,
    input::InputBundle,
    prelude::*,
    renderer::{DisplayConfig, DrawFlat2D, Pipeline, RenderBundle, Stage},
    utils::application_root_dir,
};

use crate::game::Game;

mod game;
mod fight_stick;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let display_path = format!("{}/resources/display_config.ron", application_root_dir());
    let config = DisplayConfig::load(&display_path);

    let pipe = Pipeline::build()
    .with_stage(
        Stage::with_backbuffer()
            .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
            .with_pass(DrawFlat2D::new()),
    );

    let binding_path = format!("{}/resources/binding_config.ron", application_root_dir());

    let input_bundle =
        InputBundle::<String, String>::new().with_bindings_from_file(binding_path)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(RenderBundle::new(pipe, Some(config)))?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?;
    let mut game = Application::new("./", Game, game_data)?;

    game.run();

    Ok(())
}
