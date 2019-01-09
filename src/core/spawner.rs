use crate::constants::{IMG_OGRE, LEVEL_FLOOR};
use crate::core::asset_store::AssetStore;
use crate::core::collider;
use crate::core::tick::Tick;
use ggez::graphics::{self, Vector2};
use std::collections::VecDeque;
use std::time::{Duration, Instant};

pub struct Spawner<'a> {
    pub unit: &'a graphics::Image,
    pub entities: VecDeque<Entity>,
    pub last_spawn_time: Option<Instant>,
    pub position: graphics::Point2,

    frequency: Duration,
}

impl<'a> Spawner<'a> {
    pub fn new(assets: &'a AssetStore) -> Spawner<'a> {
        let unit = assets.get_image(IMG_OGRE).unwrap();
        let entities = VecDeque::new();
        let last_spawn_time = Some(Instant::now());
        let position = graphics::Point2::new(900.0, LEVEL_FLOOR);
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
        let enemy = Entity::new(self.unit.clone(), self.position);

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

impl<'a> Tick for Spawner<'a> {
    fn update(&mut self) {
        if self.should_spawn() {
            self.spawn();
        }

        slide_enemies(&mut self.entities);

        self.remove_offscreen();
    }
}

fn slide_enemies(enemies: &mut VecDeque<Entity>) {
    for enemy in enemies {
        enemy.slide();
    }
}

pub struct Entity {
    pub unit: graphics::Image,
    pub position: graphics::Point2,

    radius: u32,
}

impl Entity {
    pub fn new(unit: graphics::Image, position: graphics::Point2) -> Entity {
        let radius = unit.width();

        Entity {
            unit,
            position,
            radius,
        }
    }

    pub fn slide(&mut self) {
        self.position += Vector2::new(-5.0, 0.0);
    }

    pub fn is_offscreen(&self) -> bool {
        self.position.coords.x <= 0.0
    }
}

impl collider::SphereCollider for Entity {
    fn radius(&self) -> u32 {
        self.radius
    }

    fn center(&self) -> &graphics::Point2 {
        &self.position
    }
}
