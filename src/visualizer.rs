use std::cmp::max;
use std::collections::HashMap;

use ggez::Context;
use ggez::graphics::Rect;
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
    map_size: Vec2,
    resources_instances: HashMap<Texture, InstanceArrayCoordinated>,
    instances: Vec<HashMap<Texture, InstanceArray>>,
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
        let mut instances: Vec<HashMap<Texture, InstanceArray>> = Vec::new();

        let mut diagonals = Vec::new();

        for k in 0..=(2 * map.len() - 2) as usize {
            let mut diagonal_components = Vec::new();
            for x in 0..=k {
                let y = k - x;
                if y < map.len() && x < map.len() {
                    diagonal_components.push((x, y));
                }
            }
            if !diagonal_components.is_empty() {
                diagonals.push(diagonal_components);
            }
        }

        diagonals.iter().for_each(|diagonal| {
            let mut diagonal_instances = HashMap::new();
            diagonal.iter().for_each(|(x, y)| {
                let texture = Texture::from_tile(&map[*y][*x]);

                let image_x = (texture.width() * 0.5) * (map.len() - y + x - 1) as f32;
                let image_y = ((texture.height() - 1.0) * 0.25) * (x + y) as f32;

                if !diagonal_instances.contains_key(&texture) {
                    let image = texture.get_image(gfx);
                    let instance = InstanceArray::new(gfx, image);
                    diagonal_instances.insert(texture.clone(), instance);
                }

                let instance = diagonal_instances.get_mut(&texture).unwrap();

                instance.push(
                    ggez::graphics::DrawParam::new()
                        .dest(Vec2::new(image_x, image_y))
                );
            });
            instances.push(diagonal_instances);
        });

        Self {
            map_size: vec2(map.len() as f32, map.len() as f32),
            resources_instances: HashMap::new(),
            instances,
            origin,
            image_scale,
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) -> ggez::GameResult {
        let mut canvas = Canvas::from_frame(&ctx.gfx, Color::WHITE);

        canvas.set_screen_coordinates(Rect::new(
            self.origin.x,
            self.origin.y,
            ctx.gfx.window().inner_size().width as f32,
            ctx.gfx.window().inner_size().height as f32,
        ));

        let mut sampler = Sampler::default();
        sampler.mag = FilterMode::Nearest;
        sampler.min = FilterMode::Nearest;
        canvas.set_sampler(sampler);

        self.instances
            .iter()
            .enumerate()
            .for_each(|(y, row)| {
                row.iter()
                    .for_each(|(_, instance)| {
                        let row_position = 3.75 * self.image_scale * y as f32 - self.origin.y;
                        if instance.capacity() > 0 
                            && row_position + 16.0 * self.image_scale >= 0.0
                            && row_position < ctx.gfx.window().inner_size().height as f32 
                        {
                            canvas.draw(instance, ggez::graphics::DrawParam::new()
                                .scale(vec2(self.image_scale, self.image_scale)));
                        }
                    });
            });

        canvas.finish(&mut ctx.gfx)?;
        Ok(())
    }

    pub fn add_scale(&mut self, gfx: &impl Has<GraphicsContext>, scale: f32) {
 
        if self.image_scale + scale * 0.01 > 1.0 && self.image_scale + scale * 0.01 < 4.0 {

            let screen_width = gfx.retrieve().window().inner_size().width as f32;
            let screen_height = gfx.retrieve().window().inner_size().height as f32;

            self.origin.x += scale * 0.5 * 0.01;
            self.origin.y += scale * 0.5 * 0.01;

            self.image_scale += scale * 0.01;
        }
    }

    pub fn add_offset(&mut self, offset: Vec2) {
        self.origin.x += offset.x;
        self.origin.y -= offset.y;
    }

    pub fn set_center(&mut self, gfx: &impl Has<GraphicsContext>, tile_center: Vec2) {

        let x = tile_center.x; 
        let y = tile_center.y;

        let screen_width = gfx.retrieve().window().inner_size().width as f32;
        let screen_height = gfx.retrieve().window().inner_size().height as f32;

        let image_x = (16.0 * 0.5) * (self.map_size.y - y + x - 1.0) as f32;
        let image_y = 3.75 * (x + y - 1.0) as f32;

        self.origin.x = -image_x - screen_width * 0.5 + 16.0 * 0.5 * self.image_scale;
        self.origin.y = -image_y - screen_height * 0.5 + 4.0 * 0.5 * self.image_scale;
    }

    pub fn origin(&self) -> Vec2 {
        self.origin
    }

    pub fn scale(&self) -> f32 {
        self.image_scale
    }
}
