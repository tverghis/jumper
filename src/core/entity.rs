use crate::core::collider;
use ggez::graphics;

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
        self.position += graphics::Vector2::new(-5.0, 0.0);
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
