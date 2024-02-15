use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::Rc,
};

use ggez::{
    context::Has,
    glam::{vec2, Vec2},
    graphics::{Canvas, Color, DrawParam, GraphicsContext, InstanceArray},
};

use robotics_lib::world::tile::Tile;

use crate::visualizer::textures::{self, Texture};

use super::Component;

struct CoordinatedInstance {
    array: InstanceArray,
    elements: Vec<(usize, usize)>,
}

pub(crate) struct TilesMapComponent {
    map_rc: Rc<RefCell<Vec<Vec<Tile>>>>,
    diagonals: Vec<Vec<(usize, usize)>>,
    instances: Vec<HashMap<Texture, CoordinatedInstance>>,
}

pub(crate) struct TilesMapComponentParam {
    pub origin: Vec2,
    pub window_size: Vec2,
    pub scale: f32,
}

pub(crate) struct TilesMapComponentUpdateParam {
    pub current_map: Vec<Vec<Option<Tile>>>,
}

impl TilesMapComponent {
    pub fn from_map(gfx: &impl Has<GraphicsContext>, map_rc: Rc<RefCell<Vec<Vec<Tile>>>>) -> Self {
        let mut diagonals: Vec<Vec<(usize, usize)>> = Vec::new();
        let mut instances: Vec<HashMap<Texture, CoordinatedInstance>> = Vec::new();

        let map = map_rc.borrow().clone();

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
            instances.push(Self::create_diagonal_instances(
                map_rc.clone(),
                gfx,
                diagonal,
            ));
        });

        Self {
            map_rc,
            diagonals,
            instances,
        }
    }

    fn create_diagonal_instances(
        map_rc: Rc<RefCell<Vec<Vec<Tile>>>>,
        gfx: &impl Has<GraphicsContext>,
        diagonal: &Vec<(usize, usize)>,
    ) -> HashMap<Texture, CoordinatedInstance> {
        let map = map_rc.borrow();

        let mut diagonal_instances = Texture::get_blocks()
            .iter()
            .map(|texture| {
                (
                    *texture,
                    CoordinatedInstance {
                        array: InstanceArray::new(gfx, texture.get_image(gfx)),
                        elements: Vec::new(),
                    },
                )
            })
            .collect::<HashMap<_, _>>();

        diagonal.iter().for_each(|(x, y)| {
            let texture = Texture::from_tile(&map[*y][*x]);
            let image_x = (Texture::width() * 0.5) * (map.len() - y + x - 1) as f32;
            let image_y = ((Texture::height() - 1.0) * 0.25) * (x + y) as f32;

            let instance = diagonal_instances.get_mut(&texture).unwrap();

            instance.array.push(
                ggez::graphics::DrawParam::new()
                    .dest(Vec2::new(image_x, image_y))
                    .color(Color::from_rgba(0, 0, 0, 127)),
            );
            instance.elements.push((*x, *y));
        });

        diagonal_instances
    }
}

impl Component<TilesMapComponentParam, TilesMapComponentUpdateParam> for TilesMapComponent {
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
                if instance.elements.capacity() > 0
                    && row_position + 16.0 * component_param.scale >= 0.0
                    && row_position < component_param.window_size.y as f32
                {
                    canvas.draw(
                        &instance.array,
                        DrawParam::new().scale(vec2(component_param.scale, component_param.scale)),
                    );
                }
            });
        });
        Ok(())
    }

    fn update(
        &mut self,
        update_param: TilesMapComponentUpdateParam,
    ) -> Result<(), ggez::GameError> {
        let current_map = update_param.current_map();
        let map = self.map_rc.clone().borrow().clone();
        let mut updated_map = map.clone();

        // Set color white for every discovered tile
        map.iter().enumerate().for_each(|(y, row)| {
            row.iter().enumerate().for_each(|(x, tile)| {
                // If tile was discovered previously by the player
                if current_map[y][x].is_some() {
                    // Get the instance of the tile
                    let diagonal_instance = &mut self.instances[x + y];

                    // Calculate the position of the tile in the matrix
                    let image_x =
                        (Texture::width() * 0.5) * (map.len() - y + x - 1) as f32;
                    let image_y =
                        ((Texture::height() - 1.0) * 0.25) * (x + y) as f32;

                    // Get the position of the tile in the instance array
                    // Suppose that the tile exists in the elements array
                    let position = diagonal_instance
                        .get_mut(&Texture::from_tile(tile))
                        .unwrap()
                        .elements
                        .iter()
                        .position(|(e_x, e_y)| x == *e_x && y == *e_y)
                        .unwrap();

                    // Update the corrisponding draw param
                    diagonal_instance
                        .get_mut(&Texture::from_tile(tile))
                        .unwrap()
                        .array
                        .update(
                            position as u32,
                            DrawParam::new()
                                .dest(vec2(image_x, image_y))
                                .color(Color::WHITE),
                        );
                }
            });
        });

        // Update modified tiles
        map.iter()
            // Compare elements of the previous map with the current map
            .zip(current_map.iter())
            .enumerate()
            .for_each(|(y, (prev_row, last_row))| {
                prev_row
                    .iter()
                    .zip(last_row.iter())
                    .enumerate()
                    .filter_map(|(x, (prev_tile, last_tile))| {
                        // Remove not discovered tiles from the list
                        if let Some(tile) = last_tile {
                            Some((x, (prev_tile, tile)))
                        } else {
                            None
                        }
                    })
                    .for_each(|(x, (prev_tale, last_tale))| {
                        // Check if tile texture has changed

                        let prev_texture = Texture::from_tile(prev_tale);
                        let last_texture = Texture::from_tile(last_tale);

                        if prev_tale != last_tale {
                            let diagonal_instances = &mut self.instances[x + y];

                            // Get actual instance of the tile
                            // Suppose that the tile exists in the elements array
                            let prev_instance = diagonal_instances.get_mut(&prev_texture).unwrap();

                            // Get the position of the tile in the instance array
                            let element_position: usize = prev_instance
                                .elements
                                .iter()
                                .position(|(e_x, e_y)| x == *e_x && y == *e_y)
                                .unwrap()
                                .clone();

                            // Get corresponding draw param
                            let draw_param = prev_instance
                                .array
                                .instances()
                                .get(element_position)
                                .unwrap()
                                .clone();

                            // Remove the draw param slicing the array
                            prev_instance.array.set(
                                prev_instance
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
                            diagonal_instances
                                .get_mut(&last_texture)
                                .unwrap()
                                .array
                                .push(draw_param.clone());

                            // Update corrisponding tile in the map
                            updated_map[y][x] = last_tale.clone();
                        }
                    })
            });

        self.map_rc.replace(updated_map);

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

impl TilesMapComponentUpdateParam {
    pub(crate) fn new(current_map: Vec<Vec<Option<Tile>>>) -> Self {
        Self { current_map }
    }

    pub(crate) fn current_map(&self) -> &Vec<Vec<Option<Tile>>> {
        &self.current_map
    }
}
