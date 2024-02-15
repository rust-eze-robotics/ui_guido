use std::{cell::RefCell, collections::HashMap, rc::Rc};

use ggez::{
    context::Has,
    glam::vec2,
    graphics::{Color, DrawParam, GraphicsContext, InstanceArray},
};
use robotics_lib::{event::events::Event, world::tile::Tile};

use crate::visualizer::textures::Texture;

use super::Component;

pub(crate) struct ContentsMapComponent {
    map_rc: Rc<RefCell<Vec<Vec<Tile>>>>,
    instances: HashMap<Texture, ContentInstance>,
}

struct ContentInstance {
    array: InstanceArray,
    elements: Vec<(usize, usize)>,
}

pub(crate) struct ContentsMapComponentParam {
    pub scale: f32,
}

pub(crate) enum ContentsMapComponentUpdateType {
    WorldVisibility(Vec<Vec<Option<Tile>>>),
    ContentChange(Tile, (usize, usize)),
}

pub(crate) struct ContentsMapComponentUpdateParam {
    _type: ContentsMapComponentUpdateType,
    // tile: Tile,
    // coords: (usize, usize),
}

impl ContentsMapComponent {
    pub fn from_map(gfx: &impl Has<GraphicsContext>, map_rc: Rc<RefCell<Vec<Vec<Tile>>>>) -> Self {
        let mut instances = HashMap::new();

        map_rc
            .clone()
            .borrow()
            .iter()
            .enumerate()
            .for_each(|(y, row)| {
                row.iter().enumerate().for_each(|(x, tile)| {
                    if let Some(texture) = Texture::from_content(&tile.content) {
                        let image_x = (Texture::width() * 0.5) * (row.len() - y + x - 1) as f32;
                        let image_y = ((Texture::height() - 1.0) * 0.25) * (x + y) as f32;
                        let offset_y = if tile.elevation < 3 { 2.0 } else { 6.0 };

                        let instance =
                            instances.entry(texture).or_insert_with(|| ContentInstance {
                                array: InstanceArray::new(gfx, texture.get_image(gfx)),
                                elements: Vec::new(),
                            });

                        instance.array.push(
                            DrawParam::new()
                                .dest(vec2(image_x, image_y - offset_y))
                                .color(Color::from_rgba(0, 0, 0, 127)),
                        );
                        instance.elements.push((x, y));
                    }
                });
            });

        Self { instances, map_rc }
    }

    pub fn update_content(&mut self, tile: &Tile, coords: (usize, usize)) {

        let mut map = self.map_rc.borrow_mut();
        let y = coords.0;
        let x = coords.1;

        let previous_texture = Texture::from_content(&map[y][x].content).unwrap();

        let current_texture = Texture::from_content(&tile.content).unwrap();

        if let Some(instance) = self.instances.get_mut(&previous_texture) {
            // Get the position of the content in the instance array
            let element_position: usize = instance
                .elements
                .iter()
                .position(|(e_x, e_y)| x == *e_x && y == *e_y)
                .unwrap()
                .clone();

            // Get corresponding draw param
            let draw_param = instance
                .array
                .instances()
                .get(element_position)
                .unwrap()
                .clone();

            // Remove the draw param slicing the array
            instance.array.set(
                instance
                    .array
                    .instances()
                    .into_iter()
                    .enumerate()
                    .filter_map(|(i, draw_param)| {
                        if i != element_position {
                            Some(draw_param.clone())
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>(),
            );

            // Push the draw param in the new instance of the texture
            self.instances
                .get_mut(&current_texture)
                .unwrap()
                .array
                .push(draw_param.clone());
        }

        map[y][x].content = tile.content.clone();
    }
}

impl Component<ContentsMapComponentParam, ContentsMapComponentUpdateParam>
    for ContentsMapComponent
{
    fn draw(
        &self,
        canvas: &mut ggez::graphics::Canvas,
        _draw_param: DrawParam,
        component_param: ContentsMapComponentParam,
    ) -> Result<(), ggez::GameError> {
        for (_texture, instance) in &self.instances {
            canvas.draw(
                &instance.array,
                DrawParam::new().scale(vec2(component_param.scale, component_param.scale)),
            );
        }
        Ok(())
    }

    fn update(
        &mut self,
        update_param: ContentsMapComponentUpdateParam,
    ) -> Result<(), ggez::GameError> {

        match update_param._type {
            ContentsMapComponentUpdateType::WorldVisibility(current_world) => {
                
                for (y, row) in current_world.iter().enumerate() {
                    for (x, tile) in row.iter().enumerate() {
                        if let Some(tile) = tile {
                            if let Some(texture) = Texture::from_content(&tile.content) {
                                if let Some(instance) = self.instances.get_mut(&texture) {
                                    let element_position: usize = instance
                                        .elements
                                        .iter()
                                        .position(|(e_x, e_y)| x == *e_x && y == *e_y)
                                        .unwrap()
                                        .clone();

                                    let draw_param = instance
                                        .array
                                        .instances()
                                        .get(element_position)
                                        .unwrap()
                                        .clone();

                                    instance.array.update(
                                        element_position as u32,
                                        draw_param
                                            .color(Color::WHITE),
                                    );
                                }
                            }
                        }
                    }
                }
            }

            ContentsMapComponentUpdateType::ContentChange(tile, coords) => {
                self.update_content(&tile, coords);
            }
        }

        Ok(())
    }
}

impl ContentsMapComponentParam {
    pub fn new(scale: f32) -> Self {
        Self { scale }
    }
}

impl ContentsMapComponentUpdateParam {
    pub fn new(_type: ContentsMapComponentUpdateType) -> Self {
        Self { _type }
    }
}
