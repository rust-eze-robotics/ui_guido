use std::{collections::HashMap, cmp::max};

use ggez::{
    context::{Has, HasMut},
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
            map_size: vec2(map[0].len() as f32, map.len() as f32),
            resources_instances,
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

        self.resources_instances
            .iter_mut()
            .for_each(|(texture, instance_coord)| {
                instance_coord.instance.set(
                    instance_coord
                        .elements
                        .clone()
                        .iter()
                        .map(|coord| {
                            let image_x = self.origin.x
                                + (texture.width() * 0.5 * self.image_scale)
                                    * (self.map_size.x as f32 - coord.y + coord.x) as f32;
                            let image_y = self.origin.y
                                + ((texture.height() - 1.0) * 0.25 * self.image_scale)
                                    * (coord.x + coord.y) as f32;
                            ggez::graphics::DrawParam::new()
                                .dest(Vec2::new(image_x, image_y))
                                .scale(vec2(self.image_scale, self.image_scale))
                                .z(max(coord.x as i32, coord.y as i32))
                        })
                        .collect::<Vec<_>>(),
                );
                canvas.draw(
                    &instance_coord.instance,
                    ggez::graphics::DrawParam::new(),
                );
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
