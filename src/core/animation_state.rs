use ggez::graphics::Image;

const REQUEST_FRAME_THRESHOLD: u8 = 5;

pub struct AnimationState<'a> {
    sprites: &'a Vec<Image>,
    current_index: usize,
    request_frame: u8,
}

impl<'a> AnimationState<'a> {
    pub fn with_sprites(sprites: &'a Vec<Image>) -> AnimationState {
        let current_index = 0;
        let request_frame = 0;

        AnimationState {
            sprites,
            current_index,
            request_frame,
        }
    }

    pub fn frame(&self) -> &Image {
        &self.sprites[self.current_index]
    }

    pub fn advance(&mut self) {
        self.request_frame += 1;

        if self.request_frame == REQUEST_FRAME_THRESHOLD {
            self.current_index += 1;

            if self.current_index == self.sprites.len() {
                self.current_index = 0;
            }
            self.request_frame = 0;
        }
    }
}
