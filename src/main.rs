#[macro_use]
extern crate specs_derive;

use nalgebra as na;

type Point = na::Point2<f64>;

use amethyst::{
    core::transform::TransformBundle,
    input::InputBundle,
    prelude::*,
    renderer::{ColorMask, DisplayConfig, DrawFlat2D, Pipeline, RenderBundle, Stage, ALPHA},
    utils::application_root_dir,
};

use crate::game::Game;
use crate::fight_stick::{FightStickSystem, POCParseSystem, FightStick};

mod game;
mod fight_stick;
mod parser;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let display_path = format!("{}/assets/configs/display_config.ron", application_root_dir());
    let config = DisplayConfig::load(&display_path);

    let pipe = Pipeline::build()
    .with_stage(
        Stage::with_backbuffer()
            .clear_target([0., 0., 0., 1.], 1.)
            .with_pass(DrawFlat2D::new().with_transparency(ColorMask::all(), ALPHA, None)),
    );

    let binding_path = format!("{}/assets/configs/binding_config.ron", application_root_dir());

    let input_bundle =
        InputBundle::<String, String>::new().with_bindings_from_file(binding_path)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(RenderBundle::new(pipe, Some(config)).with_sprite_sheet_processor())?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with(POCParseSystem{
            stick: FightStick::new(),
            input_buffer: Vec::new(),
        }, "parse_system", &["input_system"])
        .with(FightStickSystem, "fight_stick_system", &["input_system"]);
    let mut game = Application::new("./", Game, game_data)?;

    game.run();

    Ok(())
}
