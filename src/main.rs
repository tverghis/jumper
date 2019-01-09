mod constants;
mod core;

use crate::constants::{FONT_DEFAULT, IMG_OGRE, IMG_PLAYER_JUMP, IMG_PLAYER_RUN};
use crate::core::asset_store::AssetStore;
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

    let assets = match load_assets(context) {
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

fn load_assets(context: &mut Context) -> GameResult<AssetStore> {
    let mut store = AssetStore::new();

    load_fonts(context, &mut store)?;
    load_images(context, &mut store)?;

    Ok(store)
}

fn load_fonts(context: &mut Context, store: &mut AssetStore) -> GameResult<()> {
    let default_font = graphics::Font::new(context, "/fonts/ShareTechMono.ttf", 24)?;

    store.store_font(FONT_DEFAULT, default_font);

    Ok(())
}

fn load_images(context: &mut Context, store: &mut AssetStore) -> GameResult<()> {
    let player_run_img = graphics::Image::new(context, "/player/run0.png")?;
    let player_jump_img = graphics::Image::new(context, "/player/jump0.png")?;

    let ogre_img = graphics::Image::new(context, "/enemy/ogre.png")?;

    store.store_image(IMG_PLAYER_RUN, player_run_img);
    store.store_image(IMG_PLAYER_JUMP, player_jump_img);
    store.store_image(IMG_OGRE, ogre_img);

    Ok(())
}
