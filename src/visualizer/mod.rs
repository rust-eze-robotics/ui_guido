mod components;
mod textures;

use ggez::graphics::{DrawParam, Rect};
use ggez::Context;
use ggez::{
    context::Has,
    glam::{vec2, Vec2},
    graphics::{Canvas, Color, FilterMode, GraphicsContext, Sampler},
};
use robotics_lib::runner::Runner;
use robotics_lib::utils::LibError;
use robotics_lib::world::tile::Tile;

use self::components::contents_map::{ContentsMapComponent, ContentsMapComponentParam};
use self::components::player::{PlayerComponent, PlayerComponentParam};
use self::components::tails_map::{TilesMapComponentParam, TilesMapComponent};
use self::components::Component;

pub struct Visualizer {
    runner: Runner,
    map_size: Vec2,
    origin: Vec2,
    scale: f32,
    tiles_map_component: TilesMapComponent,
    contents_map_component: ContentsMapComponent,
    player_component: PlayerComponent,
}

impl Visualizer {
    pub fn new(
        gfx: &impl Has<GraphicsContext>,
        runner: Runner,
        map: &Vec<Vec<Tile>>,
        origin: Vec2,
        scale: f32,
    ) -> Self {

        let tiles_map_component = TilesMapComponent::from_map(gfx, map);
        let contents_map_component = ContentsMapComponent::from_map(gfx, map);
        let player_component = PlayerComponent::new(gfx);

        Self {
            runner,
            map_size: vec2(map.len() as f32, map.len() as f32),
            origin,
            scale,
            tiles_map_component,
            contents_map_component,
            player_component,
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
                    self.scale,
                ),
            )
            .unwrap();

        self.contents_map_component
            .draw(
                &mut canvas,
                DrawParam::new(),
                ContentsMapComponentParam::new(self.scale),
            )
            .unwrap();

        let x = self.runner().get_robot().get_coordinate().get_col();
        let y = self.runner().get_robot().get_coordinate().get_row();
        let player_x = (PlayerComponent::texture().width() * 0.5) * (self.map_size.y as usize - y + x - 1) as f32;
        let player_y = ((PlayerComponent::texture().height() - 1.0) * 0.25) * (x + y) as f32;
        self.player_component
            .draw(
                &mut canvas,
                DrawParam::new()
                    .dest(vec2(player_x, player_y + 2.0) * self.scale)
                    .scale(vec2(self.scale, self.scale)),
                PlayerComponentParam,
            )
            .unwrap();

        canvas.finish(&mut ctx.gfx)?;
        Ok(())
    }

    pub fn add_scale(&mut self, _gfx: &impl Has<GraphicsContext>, scale: f32) {
        if self.scale + scale * 0.01 > 1.0 && self.scale + scale * 0.01 < 4.0 {

            self.origin.x += scale * 0.5 * 0.01;
            self.origin.y += scale * 0.5 * 0.01;

            self.scale += scale * 0.01;
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

        self.origin.x = -image_x - screen_width * 0.5 + 16.0 * 0.5 * self.scale;
        self.origin.y = -image_y - screen_height * 0.5 + 4.0 * 0.5 * self.scale;
    }

    pub fn next_tick(&mut self) -> Result<(), LibError>  {
        self.runner.game_tick()
    }

    pub fn runner(&self) -> &Runner {
        &self.runner
    }

    pub fn origin(&self) -> Vec2 {
        self.origin
    }

    pub fn scale(&self) -> f32 {
        self.scale
    }
}
