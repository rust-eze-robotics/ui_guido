use std::collections::{HashMap, HashSet};

use ggez::{
    context::Has,
    glam::{vec2, Vec2},
    graphics::{Canvas, DrawParam, GraphicsContext, InstanceArray, Color},
};

use robotics_lib::world::tile::Tile;

use crate::visualizer::textures::{Texture, self};

use super::Component;

struct CoordinatedInstance {
    array: InstanceArray,
    elements: Vec<(usize, usize)>,
}

pub(crate) struct TilesMapComponent {
    map: Vec<Vec<Tile>>,

    diagonals: Vec<Vec<(usize, usize)>>,
    instances: Vec<HashMap<Texture, CoordinatedInstance>>,
}

pub(crate) struct TilesMapComponentParam {
    pub origin: Vec2,
    pub window_size: Vec2,
    pub scale: f32,
}

pub(crate) struct TilesMapComponentUpdateParam {
    pub last_map: Vec<Vec<Option<Tile>>>,
}

impl TilesMapComponent {
    pub fn from_map(gfx: &impl Has<GraphicsContext>, map: Vec<Vec<Tile>>) -> Self {
        let mut diagonals: Vec<Vec<(usize, usize)>> = Vec::new();
        let mut instances: Vec<HashMap<Texture, CoordinatedInstance>> = Vec::new();

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

            instances.push(Self::create_diagonal_instances(&map, gfx, diagonal));
        });

        Self { 
            map,
            diagonals,
            instances 
        }
    }

    fn create_diagonal_instances(
        map: &Vec<Vec<Tile>>,
        gfx: &impl Has<GraphicsContext>, 
        diagonal: &Vec<(usize, usize)>
    ) -> HashMap<Texture, CoordinatedInstance> {

        let mut diagonal_instances = 
            Texture::get_blocks()
                .iter()
                .map(|texture| (*texture, CoordinatedInstance {
                    array: InstanceArray::new(gfx, texture.get_image(gfx)),
                    elements: Vec::new(),
                }))
                .collect::<HashMap<_, _>>();

        diagonal.iter().for_each(|(x, y)| {
            let texture = Texture::from_tile(&map[*y][*x]);
            let image_x = (texture.width() * 0.5) * (map.len() - y + x - 1) as f32;
            let image_y = ((texture.height() - 1.0) * 0.25) * (x + y) as f32;

            let instance = diagonal_instances.get_mut(&texture).unwrap();

            instance.array.push(ggez::graphics::DrawParam::new()
                .dest(Vec2::new(image_x, image_y))
                .color(Color::from_rgba(0, 0, 0, 127))
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
                        DrawParam::new()
                            .scale(vec2(component_param.scale, component_param.scale)),
                    );
                }
            });
        });
        Ok(())
    }

    fn update(
        &mut self,
        update_param: TilesMapComponentUpdateParam
    ) -> Result<(), ggez::GameError> {
    
        let last_map = update_param.last_map();
        let mut map = self.map.clone();

        self.map
            .iter()
            .enumerate()
            .for_each(|(y, row)| {
                row.iter()
                    .enumerate()
                    .for_each(|(x, tile)| {
                        if last_map[y][x].is_some() {
                            let diagonal_instance = &mut self.instances[x + y];
                            let image_x = (Texture::from_tile(tile).width() * 0.5) * (self.map.len() - y + x - 1) as f32;
                            let image_y = ((Texture::from_tile(tile).height() - 1.0) * 0.25) * (x + y) as f32;
                            let position = diagonal_instance
                                .get_mut(&Texture::from_tile(tile))
                                .unwrap()
                                .elements
                                .iter()
                                .position(|(e_x, e_y)| x == *e_x && y == *e_y)
                                .unwrap();
                            diagonal_instance
                                .get_mut(&Texture::from_tile(tile))
                                .unwrap()
                                .array
                                .update(
                                    position as u32, 
                                    DrawParam::new()
                                        .dest(vec2(image_x, image_y))
                                        .color(Color::WHITE)
                                );
                        }

                    });
            });
       
        self.map
            .iter()
            .zip(last_map.iter())
            .enumerate()
            .for_each(|(y, (prev_row, last_row))| {
                prev_row.iter()
                    .zip(last_row.iter())
                    .enumerate()
                    .filter_map(|(x, (prev_tail, last_tail))| {
                        if let Some(tile) = last_tail {
                            Some((x, (prev_tail, tile)))
                        }
                        else {
                            None
                        }
                    })
                    .for_each(|(x, (prev_tale, last_tale))| {
                        let prev_texture = Texture::from_tile(prev_tale);
                        let last_texture = Texture::from_tile(last_tale);

                        if prev_tale != last_tale {
                            let diagonal_instances = &mut self.instances[x + y];

                            let prev_instance = diagonal_instances.get_mut(&prev_texture).unwrap();

                            let element_position: usize = prev_instance.elements
                                .iter()
                                .position(|(e_x, e_y)| x == *e_x && y == *e_y)
                                .unwrap().clone();

                            let draw_param = prev_instance.array.instances()
                                .get(element_position)
                                .unwrap()
                                .clone();

                            prev_instance.array.set(
                                prev_instance.array.instances()
                                    .into_iter()
                                    .enumerate()
                                    .filter_map(|(i, draw_param)| {
                                        if i != element_position {
                                            Some(draw_param.clone())
                                        }
                                        else {
                                            None
                                        }
                                    })
                                    .collect::<Vec<_>>()
                            );

                            diagonal_instances
                                .get_mut(&last_texture)
                                .unwrap()
                                .array.push(draw_param.clone());

                            map[y][x] = last_tale.clone();
                        }
                    })
            });

        self.map = map;

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
        Self { last_map: current_map }
    }

    pub(crate) fn last_map(&self) -> &Vec<Vec<Option<Tile>>> {
        &self.last_map
    }
}
