use std::collections::HashMap;

use ggez::{
    context::Has,
    glam::{vec2, Vec2},
    graphics::{Canvas, DrawParam, GraphicsContext, InstanceArray},
};
use robotics_lib::world::tile::Tile;

use crate::visualizer::textures::Texture;

use super::Component;

pub(crate) struct TilesMapComponent {
    instances: Vec<HashMap<Texture, InstanceArray>>,
}

pub(crate) struct TilesMapComponentParam {
    pub origin: Vec2,
    pub window_size: Vec2,
    pub scale: f32,
}

impl TilesMapComponent {
    pub fn from_map(gfx: &impl Has<GraphicsContext>, map: &Vec<Vec<Tile>>) -> Self {
        let mut diagonals: Vec<Vec<(usize, usize)>> = Vec::new();
        let mut instances: Vec<HashMap<Texture, InstanceArray>> = Vec::new();

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

                let instance = diagonal_instances.entry(texture).or_insert_with(|| 
                    InstanceArray::new(gfx, texture.get_image(gfx))
                );

                instance.push(ggez::graphics::DrawParam::new().dest(Vec2::new(image_x, image_y)));
            });

            instances.push(diagonal_instances);
        });

        Self { instances }
    }
}

impl Component<TilesMapComponentParam> for TilesMapComponent {
    fn draw(
        &self,
        canvas: &mut Canvas,
        _draw_param: DrawParam,
        component_param: TilesMapComponentParam,
    ) -> Result<(), ggez::GameError> {
        self.instances.iter().enumerate().for_each(|(y, row)| {
            row.iter().for_each(|(_, instance)| {
                let row_position =
                    3.75 * component_param.scale * y as f32 - component_param.origin.y;
                if instance.capacity() > 0
                    && row_position + 16.0 * component_param.scale >= 0.0
                    && row_position < component_param.window_size.y as f32
                {
                    canvas.draw(
                        instance,
                        DrawParam::new()
                            .scale(vec2(component_param.scale, component_param.scale)),
                    );
                }
            });
        });
        Ok(())
    }
}

impl TilesMapComponentParam {
    pub(crate) fn new(origin: Vec2, window_size: Vec2, scale: f32) -> Self {
        Self {
            origin,
            window_size,
            scale,
        }
    }
}
