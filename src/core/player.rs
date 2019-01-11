use crate::constants::{
    IMG_PLAYER_JUMP, IMG_PLAYER_RUN, LEVEL_FLOOR, MAX_JUMPS, THRUST, TIME_PER_FRAME,
};
use crate::core::animation_state::AnimationState;
use crate::core::asset_store::AssetStore;
use crate::core::collider;
use crate::core::tick::Tick;
use ggez::graphics::{Image, Point2, Vector2};

pub struct Player<'a> {
    pub position: Point2,

    jumps: u8,
    state: PlayerState,
    assets: &'a AssetStore,
    velocity: Vector2,
    anim_state: AnimationState<'a>,
}

#[derive(PartialEq)]
enum PlayerState {
    Run,
    Jump,
}

impl<'a> Player<'a> {
    pub fn new(assets: &AssetStore) -> Player {
        let position = Point2::new(50.0, LEVEL_FLOOR);
        let jumps = 0;
        let state = PlayerState::Run;
        let velocity = Vector2::new(0.0, 0.0);
        let anim_state =
            AnimationState::with_sprites(assets.get_anim_images(IMG_PLAYER_RUN).unwrap());

        Player {
            position,
            jumps,
            state,
            assets,
            velocity,
            anim_state,
        }
    }

    pub fn unit(&'a self) -> &'a Image {
        self.anim_state.frame()
    }

    pub fn jump(&mut self) {
        if self.jumps >= MAX_JUMPS {
            return;
        }

        self.velocity = Vector2::new(0.0, -50.0);
        self.jumps += 1;
    }

    fn update_position(&mut self) {
        let gravity = Vector2::new(0.0, 2.0);
        self.velocity += gravity;

        self.position += self.velocity * TIME_PER_FRAME * THRUST;

        if self.position.y > LEVEL_FLOOR {
            self.reset_dynamics();
            self.transistion_to_state(PlayerState::Run);
        } else {
            self.transistion_to_state(PlayerState::Jump);
        }
    }

    fn reset_dynamics(&mut self) {
        self.position.y = LEVEL_FLOOR;
        self.velocity = Vector2::new(0.0, 0.0);
        self.jumps = 0;
    }

    fn transistion_to_state(&mut self, target_state: PlayerState) {
        if self.state == target_state {
            return;
        }

        self.state = target_state;

        let anim_key = match self.state {
            PlayerState::Run => IMG_PLAYER_RUN,
            PlayerState::Jump => IMG_PLAYER_JUMP,
        };

        self.anim_state =
            AnimationState::with_sprites(self.assets.get_anim_images(anim_key).unwrap())
    }
}

impl<'a> Tick for Player<'a> {
    fn update(&mut self) {
        self.anim_state.advance();
        self.update_position();
    }
}

impl<'a> collider::SphereCollider for Player<'a> {
    fn radius(&self) -> u32 {
        16
    }

    fn center(&self) -> &Point2 {
        &self.position
    }
}
