extern crate amethyst;

use amethyst::prelude::*;
use amethyst::renderer::{DisplayConfig, DrawSprite, Pipeline, RenderBundle, Stage};
use amethyst::utils::application_root_dir;

pub struct Pong;

impl<'a, 'b> SimpleState<'a, 'b> for Pong {}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir();

    let path = format!(
        "{}/examples/pong_tutorial_01/resources/display_config.ron",
        app_root
    );
    let config = DisplayConfig::load(&path);

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
            .with_pass(DrawSprite::new()),
    );
    let game_data = GameDataBuilder::default()
        .with_bundle(RenderBundle::new(pipe, Some(config)).with_sprite_sheet_processor())?;

    // This line is not mentioned in the pong tutorial as it is specific to the context
    // of the git repository. It only is a different location to load the assets from.
    let assets_dir = format!("{}/examples/assets/", app_root);

    let mut game = Application::new(assets_dir, Pong, game_data)?;
    game.run();
    Ok(())
}
