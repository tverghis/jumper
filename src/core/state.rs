use crate::constants::FONT_DEFAULT;
use crate::core::asset_store::AssetStore;
use crate::core::collider::SphereCollider;
use crate::core::player::Player;
use crate::core::spawner::Spawner;
use crate::core::tick::Tick;
use ggez::event::{self, Keycode, Mod};
use ggez::graphics::{self, Font, Point2};
use ggez::{timer, Context, GameResult};

const TARGET_FPS: u32 = 60;

pub struct MainState<'a> {
    player: Player<'a>,
    enemy_spawner: Spawner<'a>,
    game_state: GameState,
    score: usize,
    assets: &'a AssetStore,
}

enum GameState {
    Playing,
    GameOver,
}

impl<'a> MainState<'a> {
    pub fn new(assets: &'a AssetStore) -> MainState<'a> {
        let player = Player::new(assets);
        let enemy_spawner = Spawner::new(assets);
        let game_state = GameState::Playing;
        let score = 0;

        MainState {
            player,
            enemy_spawner,
            game_state,
            score,
            assets,
        }
    }
}

impl<'a> event::EventHandler for MainState<'a> {
    fn update(&mut self, context: &mut Context) -> GameResult<()> {
        if let GameState::GameOver = self.game_state {
            return Ok(());
        }

        while timer::check_update_time(context, TARGET_FPS) {
            self.player.update();
            self.enemy_spawner.update();

            if self
                .enemy_spawner
                .entities
                .iter()
                .any(|enemy| self.player.collide(enemy))
            {
                self.game_state = GameState::GameOver;
                return Ok(());
            }
        }

        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult<()> {
        graphics::clear(context);

        let font = self.assets.get_font(FONT_DEFAULT).unwrap();

        match self.game_state {
            GameState::Playing => draw_game_state(context, &self, &font)?,
            GameState::GameOver => draw_game_over_state(context, &self, font)?,
        };

        graphics::present(context);

        timer::yield_now();
        Ok(())
    }

    fn key_down_event(
        &mut self,
        context: &mut Context,
        keycode: Keycode,
        _keymod: Mod,
        repeat: bool,
    ) {
        if repeat {
            return;
        }

        match keycode {
            Keycode::Space => {
                self.player.jump();
            }
            Keycode::Escape => context.quit().unwrap(),
            _ => (),
        }
    }
}

fn draw_game_state(context: &mut Context, state: &MainState, font: &Font) -> GameResult<()> {
    draw_score(context, state.score, font)?;

    let player = &state.player;
    let position = Point2::new(player.position.x, player.position.y);
    graphics::draw(context, player.unit, position, 0.0)?;

    for enemy in state.enemy_spawner.entities.iter() {
        graphics::draw(context, &enemy.unit, enemy.position, 0.0)?;
    }

    Ok(())
}

// TODO: Only generate the text if the score has changed
fn draw_score(context: &mut Context, score: usize, font: &Font) -> GameResult<()> {
    let text = get_score_text(context, score, font)?;

    let dest_point = graphics::Point2::new(10.0, 10.0);
    graphics::draw(context, &text, dest_point, 0.0)?;

    Ok(())
}

fn get_score_text(context: &mut Context, score: usize, font: &Font) -> GameResult<graphics::Text> {
    let score_text = format!("Score: {}", score);
    graphics::Text::new(context, &score_text, font)
}

fn draw_game_over_state(context: &mut Context, state: &MainState, font: &Font) -> GameResult<()> {
    let game_over_text = graphics::Text::new(context, "Game Over!", font)?;
    let score_text = graphics::Text::new(context, &format!("Your score: {}", state.score), font)?;

    graphics::draw(
        context,
        &game_over_text,
        graphics::Point2::new(480.0, 360.0),
        0.0,
    )?;

    graphics::draw(
        context,
        &score_text,
        graphics::Point2::new(480.0, 400.0),
        0.0,
    )?;

    Ok(())
}
