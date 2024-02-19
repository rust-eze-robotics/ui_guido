use std::{cell::RefCell, collections::HashMap, rc::Rc};

use ggez::{
    context::Has,
    glam::vec2,
    graphics::{Color, DrawParam, GraphicsContext, InstanceArray},
};
use robotics_lib::world::tile::Tile;

use crate::visualizer::textures::Texture;

use super::{Component, CoordinatedInstance};

/// The struct contains the state of the content map component.
pub(in crate::visualizer) struct ContentsMapComponent {
    map_rc: Rc<RefCell<Vec<Vec<Tile>>>>,
    instances: HashMap<Texture, CoordinatedInstance>,
}

/// The struct contains the parameters for drawing the component.
pub(in crate::visualizer) struct ContentsMapComponentParam {
    scale: f32,
}

/// The union contains the types of updates for the component.
pub(in crate::visualizer) enum ContentsMapComponentUpdateType {
    WorldVisibility(Vec<Vec<Option<Tile>>>),
    ContentChange(Tile, (usize, usize)),
}

/// The struct contains the parameters for updating the component.
pub(in crate::visualizer) struct ContentsMapComponentUpdateParam {
    _type: ContentsMapComponentUpdateType,
}

impl ContentsMapComponent {
    /// The constructor creates a new instance of the component from the shared reference to the
    /// map.
    pub fn from_map(gfx: &impl Has<GraphicsContext>, map_rc: Rc<RefCell<Vec<Vec<Tile>>>>) -> Self {
        // Fills the following hashmap with the instances of the textures for every tyle type.
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
                            instances
                                .entry(texture)
                                .or_insert_with(|| CoordinatedInstance {
                                    array: InstanceArray::new(gfx, texture.get_image(gfx)),
                                    elements: Vec::new(),
                                });

                        // Pushes the draw param in the instance of the texture and adds the
                        // coordinates to the elements vector.
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

    /// The function handles event of content change. Is called internally by the update method.
    fn update_content(&mut self, tile: &Tile, coords: (usize, usize)) {
        // Borrow the mutable reference to the map
        let mut map = self.map_rc.borrow_mut();
        let y = coords.0; // row
        let x = coords.1; // column

        let previous_texture = Texture::from_content(&map[y][x].content);

        let current_texture = Texture::from_content(&tile.content);

        if let (Some(previous_texture), Some(current_texture)) = (previous_texture, current_texture)
        {
            if let Some(instance) = self.instances.get_mut(&previous_texture) {
                // Gets the position of the content in the instance array.
                let element_position: usize = instance
                    .elements
                    .iter()
                    .position(|(e_x, e_y)| x == *e_x && y == *e_y)
                    .unwrap()
                    .clone();

                // Gets corresponding draw param.
                let draw_param = instance
                    .array
                    .instances()
                    .get(element_position)
                    .unwrap()
                    .clone();

                // Removes the draw param slicing the array.
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

                // Pushes the draw param in the new instance of the texture.
                self.instances
                    .get_mut(&current_texture)
                    .unwrap()
                    .array
                    .push(draw_param.clone());
            }

            // Edits the content of the map.
            map[y][x].content = tile.content.clone();
        }
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
        // Draws the instances of the textures.
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
                // For every tile in the world, updates the corresponding draw param in the instance
                for (y, row) in current_world.iter().enumerate() {
                    for (x, tile) in row.iter().enumerate() {
                        // If the tile is not None, robots knows it and it's visible.
                        // So, it updates the corrisponding the draw param.
                        if let Some(tile) = tile {
                            if let Some(texture) = Texture::from_content(&tile.content) {
                                if let Some(instance) = self.instances.get_mut(&texture) {
                                    // Gets the position of the considered content in the instance array.
                                    let element_position: usize = instance
                                        .elements
                                        .iter()
                                        .position(|(e_x, e_y)| x == *e_x && y == *e_y)
                                        .unwrap()
                                        .clone();

                                    // Gets the corresponding draw param from the instance array.
                                    let draw_param = instance
                                        .array
                                        .instances()
                                        .get(element_position)
                                        .unwrap()
                                        .clone();

                                    // Updates the draw param in the instance array with the
                                    // white color, which clears the previous grey effect.
                                    instance.array.update(
                                        element_position as u32,
                                        draw_param.color(Color::WHITE),
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
    /// The constructor creates a new instance of the parameters for the component.
    pub(in crate::visualizer) fn new(scale: f32) -> Self {
        Self { scale }
    }
}

impl ContentsMapComponentUpdateParam {
    /// The constructor creates a new instance of the parameters for updating the component.
    pub(in crate::visualizer) fn new(_type: ContentsMapComponentUpdateType) -> Self {
        Self { _type }
    }
}
