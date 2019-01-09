use crate::constants::{IMG_PLAYER_JUMP, IMG_PLAYER_RUN, LEVEL_FLOOR, THRUST, TIME_PER_FRAME};
use crate::core::asset_store::AssetStore;
use crate::core::collider;
use ggez::graphics::{Image, Point2, Vector2};

pub struct Player<'a> {
    pub unit: &'a Image,
    pub position: Point2,
    pub jumps: u8,

    state: PlayerState,
    assets: &'a AssetStore,
}

enum PlayerState {
    Run,
    Jump,
}

impl<'a> Player<'a> {
    pub fn new(assets: &AssetStore) -> Player {
        let unit = assets.get_image(IMG_PLAYER_RUN).unwrap();
        let position = Point2::new(50.0, LEVEL_FLOOR);
        let jumps = 0;

        let state = PlayerState::Run;

        Player {
            unit,
            position,
            jumps,
            state,
            assets,
        }
    }

    pub fn update_position(&mut self, velocity: Vector2) {
        self.position += velocity * TIME_PER_FRAME * THRUST;

        // Don't allow the player to drop below the floor
        // Reset the available jumps
        if self.position.y > LEVEL_FLOOR {
            self.position.y = LEVEL_FLOOR;
            self.jumps = 0;
            self.transistion_to_state(PlayerState::Run);
        } else {
            self.transistion_to_state(PlayerState::Jump);
        }
    }

    fn transistion_to_state(&mut self, target_state: PlayerState) {
        self.state = target_state;

        let unit_img = match self.state {
            PlayerState::Run => IMG_PLAYER_RUN,
            PlayerState::Jump => IMG_PLAYER_JUMP,
        };

        self.unit = self.assets.get_image(unit_img).unwrap();
    }
}

impl<'a> collider::SphereCollider for Player<'a> {
    fn radius(&self) -> f32 {
        18.0
    }

    fn center(&self) -> &Point2 {
        &self.position
    }
}
