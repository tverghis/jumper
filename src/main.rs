mod constants;
mod core;
mod util;

use crate::core::state;
use ggez::{conf, event, graphics, Context, GameResult};
use std::path;

fn main() {
    let context = &mut match create_game_context() {
        Ok(ctx) => ctx,
        Err(e) => {
            println!("Error building game context: {}", e);
            return;
        }
    };

    let assets = match util::load_assets(context) {
        Ok(assets) => assets,
        Err(e) => {
            println!("Error loading assets: {}", e);
            return;
        }
    };

    let state = &mut state::MainState::new(&assets);

    graphics::set_background_color(context, graphics::Color::new(0.0, 0.0, 0.0, 1.0));

    if let Err(err) = event::run(context, state) {
        println!("Error starting the game loop: {}", err);
    };
}

fn create_game_context() -> GameResult<Context> {
    let resource_dir = path::PathBuf::from("./resources");

    let cb = ggez::ContextBuilder::new("jumper", "tverghis")
        .add_resource_path(resource_dir)
        .window_setup(conf::WindowSetup::default().title("Jumper!"))
        .window_mode(conf::WindowMode::default().dimensions(960, 720));

    cb.build()
}
