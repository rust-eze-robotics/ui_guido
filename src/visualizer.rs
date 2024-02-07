use ggez::{
    context::{Has, HasMut},
    glam::{Vec2, vec2},
    graphics::{Canvas, Color, FilterMode, GraphicsContext, Image, Sampler},
    input::gamepad::gilrs::Filter,
};

pub struct Visualizer {
    images_matrix: Vec<Vec<Image>>,
    origin: Vec2,
    image_scale: f32,
}

impl Visualizer {
    pub fn new(images_matrix: Vec<Vec<Image>>, origin: Vec2, image_scale: f32) -> Self {
        Self {
            images_matrix,
            origin,
            image_scale,
        }
    }

    pub fn draw<T>(&mut self, gfx: &mut T) -> ggez::GameResult
    where
        T: HasMut<GraphicsContext> + Has<GraphicsContext>,
    {
        let mut canvas = Canvas::from_frame(gfx, Color::WHITE);

        let mut sampler = Sampler::default();
        sampler.mag = FilterMode::Nearest;
        sampler.min = FilterMode::Nearest;
        canvas.set_sampler(sampler);

        self.images_matrix.iter().enumerate().for_each(|(y, row)| {
            row.iter().enumerate().for_each(|(x, image)| {
                let image_x = self.origin.x
                    + (image.width() as f32 * 0.5 * self.image_scale)
                        * (self.images_matrix.len() - y + x) as f32;
                let image_y = self.origin.y
                    + ((image.height() - 1) as f32 * 0.25 * self.image_scale) * (x + y) as f32;
                let draw_param = ggez::graphics::DrawParam::new()
                    .dest(Vec2::new(image_x, image_y))
                    .scale(vec2(self.image_scale, self.image_scale));
                canvas.draw(image, draw_param);
            });
        });
        canvas.finish(gfx)?;
        Ok(())
    }

    pub fn add_scale(&mut self, scale: f32) {
        self.image_scale += scale * 0.1;
    }

    pub fn add_offset(&mut self, offset: Vec2) {
        self.origin += offset;
    }
}
