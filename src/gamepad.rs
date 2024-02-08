use ggez::glam::{vec2, Vec2};

pub struct GamePad {
    offset_leftstick_x: f32,
    offset_leftstick_y: f32,
    offset_rightstick_x: f32,
    offset_rightstick_y: f32,
}

impl GamePad {
    pub fn new() -> Self {
        Self {
            offset_leftstick_x: 0.0,
            offset_leftstick_y: 0.0,
            offset_rightstick_x: 0.0,
            offset_rightstick_y: 0.0,
        }
    }

    pub fn set_leftstick_x_offset(&mut self, offset_leftstick_x: f32) {
        self.offset_leftstick_x = -offset_leftstick_x * 5.0;
    }

    pub fn set_leftstick_y_offset(&mut self, offset_leftstick_y: f32) {
        self.offset_leftstick_y = offset_leftstick_y * 5.0;
    }

    pub fn get_leftstick_offset(&self) -> Vec2 {
        vec2(self.offset_leftstick_x, self.offset_leftstick_y)
    }

    pub fn set_rightstick_x_offset(&mut self, offset_rightstick_x: f32) {
        self.offset_rightstick_x = -offset_rightstick_x * 5.0;
    }

    pub fn set_rightstick_y_offset(&mut self, offset_rightstick_y: f32) {
        self.offset_rightstick_y = offset_rightstick_y * 5.0;
    }

    pub fn get_rightstick_offset(&self) -> Vec2 {
        vec2(self.offset_rightstick_x, self.offset_rightstick_y)
    }
}
