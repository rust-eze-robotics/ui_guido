use std::collections::HashMap;

use ggez::Context;
use ggez::{
    context::Has,
    glam::{vec2, Vec2},
    graphics::{Canvas, Color, FilterMode, GraphicsContext, InstanceArray, Sampler},
};
use robotics_lib::world::tile::Tile;

use crate::textures::Texture;

struct InstanceArrayCoordinated {
    instance: InstanceArray,
    elements: Vec<Vec2>,
}

pub struct Visualizer {
    map: Vec<Vec<Tile>>,
    map_size: Vec2,
    resources_instances: HashMap<Texture, InstanceArrayCoordinated>,
    origin: Vec2,
    image_scale: f32,
}

impl Visualizer {
    pub fn new(
        gfx: &impl Has<GraphicsContext>,
        map: &Vec<Vec<Tile>>,
        origin: Vec2,
        image_scale: f32,
    ) -> Self {
        let mut resources_instances: HashMap<Texture, InstanceArrayCoordinated> = HashMap::new();

        map.iter().enumerate().for_each(|(y, row)| {
            row.iter().enumerate().for_each(|(x, tile)| {
                let texture = Texture::from_tile(tile);
                if !resources_instances.contains_key(&texture) {
                    let image = texture.get_image(gfx);
                    let instance = InstanceArray::new(gfx, image);
                    resources_instances.insert(
                        texture.clone(),
                        InstanceArrayCoordinated {
                            instance,
                            elements: Vec::new(),
                        },
                    );
                }

                let instance = resources_instances.get_mut(&texture).unwrap();
                instance.elements.push(vec2(x as f32, y as f32));
            });
        });

        Self {
            map: map.clone(),
            map_size: vec2(map[0].len() as f32, map.len() as f32),
            resources_instances,
            origin,
            image_scale,
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) -> ggez::GameResult {
        let mut canvas = Canvas::from_frame(&ctx.gfx, Color::WHITE);

        let mut sampler = Sampler::default();
        sampler.mag = FilterMode::Nearest;
        sampler.min = FilterMode::Nearest;
        canvas.set_sampler(sampler);

        let mut diagonals = Vec::new();

        for k in 0..=(self.map_size.x + self.map_size.y - 2.0) as usize {
            let mut diagonal_components = Vec::new();
            for x in 0..=k {
                let y = k - x;
                if y < self.map_size.y as usize && x < self.map_size.x as usize {
                    diagonal_components.push((x, y));
                }
            }
            if !diagonal_components.is_empty() {
                diagonals.push(diagonal_components);
            }
        }

        diagonals.iter().for_each(|diagonal| {
            diagonal.iter().for_each(|(x, y)| {
                let texture = Texture::from_tile(&self.map[*y][*x]);
                let instance = &mut self.resources_instances.get_mut(&texture).unwrap().instance;

                let image_x = self.origin.x
                    + (texture.width() * 0.5 * self.image_scale)
                        * (self.map_size.x as usize - y + x) as f32;
                let image_y = self.origin.y
                    + ((texture.height() - 1.0) * 0.25 * self.image_scale) * (x + y) as f32;

                if image_x + texture.width() * self.image_scale > 0.0
                    && image_x < ctx.gfx.window().inner_size().width as f32
                    && image_y + texture.height() * self.image_scale > 0.0
                    && image_y < ctx.gfx.window().inner_size().height as f32
                {
                    instance.push(
                        ggez::graphics::DrawParam::new()
                            .dest(Vec2::new(image_x, image_y))
                            .scale(vec2(self.image_scale, self.image_scale)),
                    );
                }
            });

            for (_, instance_coord) in &mut self.resources_instances {
                if instance_coord.instance.capacity() > 0 {
                    canvas.draw(&instance_coord.instance, ggez::graphics::DrawParam::new());
                    instance_coord.instance.clear();
                }
            }
        });

        canvas.finish(&mut ctx.gfx)?;
        Ok(())
    }

    pub fn add_scale(&mut self, gfx: &impl Has<GraphicsContext>, scale: f32) {
        if self.image_scale + scale * 0.01 > 0.5 && self.image_scale + scale * 0.01 < 4.0 {
            let screen_width = gfx.retrieve().window().inner_size().width as f32;
            let screen_height = gfx.retrieve().window().inner_size().height as f32;

            let offset_x = (screen_width * 0.5 - self.origin.x)
                / (16.0 * self.map_size.x as f32 * self.image_scale);
            let offset_y = (screen_height * 0.5 - self.origin.y)
                / (8.0 * self.map_size.y as f32 * self.image_scale);

            self.origin.x -= self.map_size.x * scale * 0.01 * 16.0 * offset_x;
            self.origin.y -= self.map_size.y * scale * 0.01 * 8.0 * offset_y;

            self.image_scale += scale * 0.01;
        }
    }

    pub fn add_offset(&mut self, offset: Vec2) {
        self.origin += offset;
    }
}
