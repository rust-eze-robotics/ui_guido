mod components;
mod textures;

use ggez::graphics::{DrawParam, Rect};
use ggez::Context;
use ggez::{
    context::Has,
    glam::{vec2, Vec2},
    graphics::{Canvas, Color, FilterMode, GraphicsContext, Sampler},
};
use robotics_lib::world::tile::Tile;

use self::components::contents_map::{ContentsMapComponent, ContentsMapComponentParam};
use self::components::tails_map::{TilesMapComponentParam, TilesMapComponent};
use self::components::Component;

pub struct Visualizer {
    map_size: Vec2,
    origin: Vec2,
    image_scale: f32,
    tiles_map_component: TilesMapComponent,
    contents_map_component: ContentsMapComponent,
}

impl Visualizer {
    pub fn new(
        gfx: &impl Has<GraphicsContext>,
        map: &Vec<Vec<Tile>>,
        origin: Vec2,
        image_scale: f32,
    ) -> Self {

        let tiles_map_component = TilesMapComponent::from_map(gfx, map);
        let contents_map_component = ContentsMapComponent::from_map(gfx, map);

        Self {
            map_size: vec2(map.len() as f32, map.len() as f32),
            origin,
            image_scale,
            tiles_map_component,
            contents_map_component
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

        self.tiles_map_component
            .draw(
                &mut canvas,
                DrawParam::new(),
                TilesMapComponentParam::new(
                    self.origin(),
                    vec2(
                        ctx.gfx.window().inner_size().width as f32,
                        ctx.gfx.window().inner_size().height as f32,
                    ),
                    self.image_scale,
                ),
            )
            .unwrap();

        self.contents_map_component
            .draw(
                &mut canvas,
                DrawParam::new(),
                ContentsMapComponentParam::new(self.image_scale),
            )
            .unwrap();

        canvas.finish(&mut ctx.gfx)?;
        Ok(())
    }

    pub fn add_scale(&mut self, _gfx: &impl Has<GraphicsContext>, scale: f32) {
        if self.image_scale + scale * 0.01 > 1.0 && self.image_scale + scale * 0.01 < 4.0 {
            // let screen_width = gfx.retrieve().window().inner_size().width as f32;
            // let screen_height = gfx.retrieve().window().inner_size().height as f32;

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
