use crate::constants::{IMG_OGRE, LEVEL_FLOOR};
use crate::core::asset_store::AssetStore;
use crate::core::collider;
use ggez::graphics::{self, Point2, Vector2};
use std::collections::VecDeque;
use std::time::{Duration, Instant};

pub struct Spawner<'a> {
    pub unit: &'a graphics::Image,
    pub entities: VecDeque<Enemy>,
    pub last_spawn_time: Option<Instant>,
    pub position: Point2,

    frequency: Duration,
}

impl<'a> Spawner<'a> {
    pub fn new(assets: &'a AssetStore) -> Spawner<'a> {
        let unit = assets.get_image(IMG_OGRE).unwrap();
        let entities = VecDeque::new();
        let last_spawn_time = Some(Instant::now());
        let position = Point2::new(900.0, LEVEL_FLOOR);
        let frequency = Duration::from_millis(1000);

        Spawner {
            unit,
            entities,
            last_spawn_time,
            position,
            frequency,
        }
    }

    pub fn spawn(&mut self) {
        let enemy = Enemy {
            unit: self.unit.clone(),
            position: self.position,
        };

        self.entities.push_back(enemy);

        self.last_spawn_time = Some(Instant::now());
    }

    pub fn should_spawn(&self) -> bool {
        match self.last_spawn_time {
            Some(time) => time.elapsed() >= self.frequency,
            None => true,
        }
    }

    pub fn remove_offscreen(&mut self) -> bool {
        let first_unit = self.entities.front();

        if let Some(unit) = first_unit {
            if unit.is_offscreen() {
                self.entities.pop_front();
                return true;
            }
        }

        false
    }
}

pub struct Enemy {
    pub unit: graphics::Image,
    pub position: graphics::Point2,
}

impl Enemy {
    pub fn slide(&mut self) {
        self.position += Vector2::new(-5.0, 0.0);
    }

    pub fn is_offscreen(&self) -> bool {
        self.position.coords.x <= 0.0
    }
}

impl collider::SphereCollider for Enemy {
    fn radius(&self) -> f32 {
        16.0
    }

    fn center(&self) -> &Point2 {
        &self.position
    }
}
