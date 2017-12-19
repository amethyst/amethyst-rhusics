extern crate amethyst;
extern crate genmesh;
extern crate rand;
extern crate rhusics;
extern crate amethyst_rhusics;

mod bundle;
mod emission;
mod resources;
mod state;

use amethyst::core::TransformBundle;
use amethyst::prelude::*;
use amethyst::renderer::{DisplayConfig, DrawFlat, Pipeline, PosTex, RenderBundle, RenderSystem,
                         Stage};
use amethyst::utils::fps_counter::FPSCounterBundle;

use self::bundle::SimulationBundle;

fn run() -> Result<(), amethyst::Error> {
    let path = format!(
        "{}/examples/boxes3d/resources/display_config.ron",
        env!("CARGO_MANIFEST_DIR")
    );
    let config = DisplayConfig::load(&path);

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0., 0., 0., 1.0], 1.0)
            .with_pass(DrawFlat::<PosTex>::new()),
    );

    let mut game = Application::build("./", self::state::Emitting)?
        .with_bundle(FPSCounterBundle::default())?
        .with_bundle(SimulationBundle)?
        .with_bundle(TransformBundle::new().with_dep(&["movement_system"]))?
        .with_bundle(RenderBundle::new())?
        .with_local(RenderSystem::build(pipe, Some(config))?)
        .build()
        .expect("Fatal error");

    game.run();

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        println!("Error occurred during game execution: {}", e);
        ::std::process::exit(1);
    }
}

