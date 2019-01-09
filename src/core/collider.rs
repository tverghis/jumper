use ggez::graphics::Point2;
use ggez::nalgebra as na;

pub trait SphereCollider {
    fn radius(&self) -> f32;
    fn center(&self) -> &Point2;

    fn collide<T: SphereCollider>(&self, other: &T) -> bool {
        let distance = na::distance_squared(self.center(), other.center());
        distance <= (self.radius() + other.radius()).powi(2)
    }
}
