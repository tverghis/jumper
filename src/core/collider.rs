use ggez::graphics::Point2;
use ggez::nalgebra as na;

pub trait SphereCollider {
    fn radius(&self) -> u32;
    fn center(&self) -> &Point2;

    fn collide(&self, other: &impl SphereCollider) -> bool {
        let distance = na::distance_squared(self.center(), other.center());
        distance <= (self.radius() + other.radius()).pow(2) as f32
    }
}
