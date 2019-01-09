use ggez::graphics;

pub struct AnimationState {
    sprites: Vec<graphics::Image>,
}

impl AnimationState {
    pub fn new() -> AnimationState {
        let sprites = Vec::new();

        AnimationState { sprites }
    }

    pub fn add_sprite(&mut self, sprite: graphics::Image) {
        self.sprites.push(sprite);
    }

    pub fn iter(&self) -> std::iter::Cycle<std::slice::Iter<graphics::Image>> {
        self.sprites.iter().cycle()
    }
}
