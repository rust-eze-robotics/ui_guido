mod components;
mod textures;

use std::cell::RefCell;
use std::rc::Rc;

use ggez::graphics::{DrawParam, Rect};
use ggez::Context;
use ggez::{
    context::Has,
    glam::{vec2, Vec2},
    graphics::{Canvas, Color, FilterMode, GraphicsContext, Sampler},
};
use robotics_lib::event::events::Event;
use robotics_lib::runner::Runner;
use robotics_lib::utils::LibError;
use robotics_lib::world::tile::Tile;

use crate::visualizer::components::contents_map::ContentsMapComponentUpdateType;

use self::components::contents_map::{
    ContentsMapComponent, ContentsMapComponentParam, ContentsMapComponentUpdateParam,
};
use self::components::player::{PlayerComponent, PlayerComponentParam, PlayerComponentUpdateParam};
use self::components::tails_map::{
    TilesMapComponent, TilesMapComponentParam, TilesMapComponentUpdateParam,
};
use self::components::Component;

pub struct Visualizer {
    runner: Runner,
    map_rc: Rc<RefCell<Vec<Vec<Tile>>>>,
    world: Rc<RefCell<Option<Vec<Vec<Option<Tile>>>>>>,
    event_queue: Rc<RefCell<Vec<Event>>>,
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
        world: Rc<RefCell<Option<Vec<Vec<Option<Tile>>>>>>,
        event_queue: Rc<RefCell<Vec<Event>>>,
        runner: Runner,
        map_rc: Rc<RefCell<Vec<Vec<Tile>>>>,
        origin: Vec2,
        initial_position: (usize, usize),
        scale: f32,
    ) -> Self {
        let map_len = map_rc.clone().borrow().len();

        let tiles_map_component = TilesMapComponent::from_map(gfx, map_rc.clone());
        let contents_map_component = ContentsMapComponent::from_map(gfx, map_rc.clone());
        let player_component = PlayerComponent::new(gfx, initial_position, (map_len, map_len));

        Self {
            world,
            event_queue,
            runner,
            map_rc,
            map_size: vec2(map_len as f32, map_len as f32),
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

        // let x = self.runner().get_robot().get_coordinate().get_col();
        // let y = self.runner().get_robot().get_coordinate().get_row();
        // let player_x = (PlayerComponent::texture().width() * 0.5)
        //     * (self.map_size.y as usize - y + x - 1) as f32;
        // let player_y = ((PlayerComponent::texture().height() - 1.0) * 0.25) * (x + y) as f32;
        self.player_component
            .draw(
                &mut canvas,
                DrawParam::new(),
                PlayerComponentParam::new(self.scale),
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

    pub fn next_tick(&mut self) -> Result<(), LibError> {
        self.runner.game_tick()
    }

    pub fn runner(&self) -> &Runner {
        &self.runner
    }

    pub fn handle_event(&mut self) -> Result<(), LibError> {

        self.tiles_map_component
            .update(TilesMapComponentUpdateParam {
                current_map: self.world.borrow().clone().unwrap_or(vec![
                    vec![
                        None;
                        self.map_size.x
                            as usize
                    ];
                    self.map_size.y as usize
                ]),
            })
            .unwrap();

        self.contents_map_component
            .update(ContentsMapComponentUpdateParam::new(
                ContentsMapComponentUpdateType::WorldVisibility(
                    self.world.borrow().clone().unwrap_or(vec![
                        vec![
                            None;
                            self.map_size.x
                                as usize
                        ];
                        self.map_size.y as usize
                    ]),
                ),
            ))
            .unwrap();

        // Discard events while you find Event::Moved or Event::TileContentUpdated
        while let Some(event) = self.event_queue().borrow_mut().pop() {
            match event {
                Event::Moved(tile, coords) => {
                    println!("Moved to: {:?}", coords);
                    self.player_component
                        .update(PlayerComponentUpdateParam::new(coords));
                    break;
                },
                Event::TileContentUpdated(tile, coords) => {
                    println!("Tile content updated: {:?}", coords);
                    self.contents_map_component
                        .update(ContentsMapComponentUpdateParam::new(
                            ContentsMapComponentUpdateType::ContentChange(tile, coords),
                        ))
                        .unwrap();
                    break;
                },
                _ => {}
            };
        }

        Ok(())
    }

    pub fn origin(&self) -> Vec2 {
        self.origin
    }

    pub fn scale(&self) -> f32 {
        self.scale
    }

    pub fn event_queue(&self) -> Rc<RefCell<Vec<Event>>> {
        self.event_queue.clone()
    }
}
