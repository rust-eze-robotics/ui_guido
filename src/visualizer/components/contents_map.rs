use std::collections::HashMap;

use ggez::{
    context::Has,
    glam::vec2,
    graphics::{DrawParam, GraphicsContext, InstanceArray, Color},
};
use robotics_lib::world::tile::Tile;

use crate::visualizer::textures::Texture;

use super::Component;

pub(crate) struct ContentsMapComponent {
    instances: HashMap<Texture, ContentInstance>,
}

struct ContentInstance {
    array: InstanceArray,
    elements: Vec<(usize, usize)>,
}

pub(crate) struct ContentsMapComponentParam {
    pub scale: f32,
}

pub(crate) struct ContentsMapComponentUpdateParam; 

impl ContentsMapComponent {
    pub fn from_map(gfx: &impl Has<GraphicsContext>, map: &Vec<Vec<Tile>>) -> Self {
        let mut instances = HashMap::new();

        map.iter().enumerate().for_each(|(y, row)| {
            row.iter().enumerate().for_each(|(x, tile)| {
                if let Some(texture) = Texture::from_content(&tile.content) {
                    let image_x = (texture.width() * 0.5) * (map.len() - y + x - 1) as f32;
                    let image_y = ((texture.height() - 1.0) * 0.25) * (x + y) as f32;
                    let offset_y = if tile.elevation < 3 { 2.0 } else { 6.0 };

                    let instance = instances.entry(texture).or_insert_with(|| ContentInstance {
                        array: InstanceArray::new(gfx, texture.get_image(gfx)),
                        elements: Vec::new(),
                    });

                    instance.array.push(DrawParam::new()
                        .dest(vec2(image_x, image_y - offset_y))
                        .color(Color::from_rgba(0, 0, 0, 127))
                    );
                    instance.elements.push((x, y));
                }
            });
        });

        Self { instances }
    }
}

impl Component<ContentsMapComponentParam, ContentsMapComponentUpdateParam> for ContentsMapComponent {
    fn draw(
        &self,
        canvas: &mut ggez::graphics::Canvas,
        _draw_param: DrawParam,
        component_param: ContentsMapComponentParam,
    ) -> Result<(), ggez::GameError> {
        
        for (_texture, instance) in &self.instances {
            canvas.draw(
                &instance.array,
                DrawParam::new()
                    .scale(vec2(component_param.scale, component_param.scale))
            );
        }
        Ok(())
    }
}

impl ContentsMapComponentParam {
    pub fn new(scale: f32) -> Self {
        Self { 
            scale
        }
    }
}
