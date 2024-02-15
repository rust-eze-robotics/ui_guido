mod components;
mod textures;

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use ggez::graphics::{DrawParam, Rect};
use ggez::{
    context::Has,
    glam::{vec2, Vec2},
    graphics::{Canvas, Color, FilterMode, GraphicsContext, Sampler},
};
use ggez::{Context, GameResult};
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
    // Shared states
    runner: Runner,
    event_queue_rc: Rc<RefCell<VecDeque<Event>>>,
    world_rc: Rc<RefCell<Option<Vec<Vec<Option<Tile>>>>>>,

    // Visualization variables
    map_size: Vec2,
    origin: Vec2,
    scale: f32,

    // Components
    tiles_map_component: TilesMapComponent,
    contents_map_component: ContentsMapComponent,
    player_component: PlayerComponent,
}

impl Visualizer {
    /// Create a new instance of the Visualizer.
    pub fn new(
        gfx: &impl Has<GraphicsContext>,
        runner: Runner,
        world_rc: Rc<RefCell<Option<Vec<Vec<Option<Tile>>>>>>,
        event_queue_rc: Rc<RefCell<VecDeque<Event>>>,
        map_rc: Rc<RefCell<Vec<Vec<Tile>>>>,
        initial_position: (usize, usize),
        initial_scale: f32,
    ) -> Self {
        // Size of square matrix.
        let map_len = map_rc.clone().borrow().len();

        // Instance of the visualizer's components.
        let tiles_map_component = TilesMapComponent::from_map(gfx, map_rc.clone());
        let contents_map_component = ContentsMapComponent::from_map(gfx, map_rc.clone());
        let player_component = PlayerComponent::new(gfx, initial_position, (map_len, map_len));

        Self {
            runner,
            event_queue_rc,
            world_rc,
            map_size: vec2(map_len as f32, map_len as f32),
            origin: vec2(0.0, 0.0),
            scale: initial_scale,
            tiles_map_component,
            contents_map_component,
            player_component,
        }
    }

    /// The functions uses ctx for drawing the visualizer's components on the canvas.
    pub fn draw(&mut self, ctx: &mut Context) -> ggez::GameResult {
        // Initialize the canvas with lightblue background.
        let mut canvas = Canvas::from_frame(&ctx.gfx, Color::from_rgb_u32(0xADD8E6));

        // Set left-top corner into the origin.
        canvas.set_screen_coordinates(Rect::new(
            self.origin.x,
            self.origin.y,
            ctx.gfx.window().inner_size().width as f32,
            ctx.gfx.window().inner_size().height as f32,
        ));

        // Set the sampler for the canvas.
        // This is necessary to avoid the blurring of the image.
        let mut sampler = Sampler::default();
        sampler.mag = FilterMode::Nearest;
        sampler.min = FilterMode::Nearest;
        canvas.set_sampler(sampler);

        // Print the tiles component.
        self.tiles_map_component.draw(
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
        )?;

        // Print the contents component.
        self.contents_map_component.draw(
            &mut canvas,
            DrawParam::new(),
            ContentsMapComponentParam::new(self.scale),
        )?;

        // Print the player component
        self.player_component.draw(
            &mut canvas,
            DrawParam::new(),
            PlayerComponentParam::new(self.scale),
        )?;

        // Render the components on the canvas.
        canvas.finish(&mut ctx.gfx)?;

        Ok(())
    }

    /// Add zooming to the visualizer.
    pub fn add_scale(&mut self, gfx: &impl Has<GraphicsContext>, scale: f32) {
        if self.scale + scale * 0.01 > 0.5 && self.scale + scale * 0.01 < 4.0 {
            let screen_width = gfx.retrieve().window().inner_size().width as f32;
            let screen_height = gfx.retrieve().window().inner_size().height as f32;

            let offset_x =
                (screen_width * 0.5 - self.origin.x) / (16.0 * self.map_size.x as f32 * self.scale);
            let offset_y =
                (screen_height * 0.5 - self.origin.y) / (8.0 * self.map_size.y as f32 * self.scale);

            self.origin.x += self.map_size.x * scale * 0.01 * 8.0 * offset_x;
            self.origin.y += self.map_size.y * scale * 0.01 * 4.0 * offset_y;

            self.scale += scale * 0.01;
        }
    }

    /// Moves the visualizer adding an offset to the origin.
    pub fn add_offset(&mut self, offset: Vec2) {
        self.origin.x += offset.x;
        self.origin.y -= offset.y;
    }

    /// The function sets the center of the visualizer to the given tile_center.
    pub fn set_center(&mut self, gfx: &impl Has<GraphicsContext>, tile_center: Vec2) {
        let x = tile_center.x;
        let y = tile_center.y;

        let screen_width = gfx.retrieve().window().inner_size().width as f32;
        let screen_height = gfx.retrieve().window().inner_size().height as f32;

        let image_x = (16.0 * 0.5) * (self.map_size.y - y + x - 1.0) as f32;
        let image_y = 3.75 * (x + y) as f32;

        self.origin.x =
            (-image_x * self.scale + screen_width * 0.5 - 16.0 * 0.5 * self.scale) * -1.0;
        self.origin.y =
            (-image_y * self.scale + screen_height * 0.5 - 4.0 * 0.5 * self.scale) * -1.0;
    }

    /// The functions runs the next tick of the game.
    pub fn next_tick(&mut self) -> Result<(), LibError> {
        self.runner.game_tick()
    }

    /// The function updates the visualizer's components with the current state of the world.
    /// It also pops the events from the event_queue and updates the visualizer's state.
    /// If visualizer doesn't have any knowledge about the robot's known tiles, it will hide them
    /// all.
    pub fn handle_event(&mut self, gfx: &impl Has<GraphicsContext>) -> GameResult {
        self.tiles_map_component
            .update(TilesMapComponentUpdateParam::new(
                self.world_rc.borrow().clone().unwrap_or(vec![
                    vec![None; self.map_size.x as usize];
                    self.map_size.y as usize
                ]),
            ))?;

        self.contents_map_component
            .update(ContentsMapComponentUpdateParam::new(
                ContentsMapComponentUpdateType::WorldVisibility(
                    self.world_rc.borrow().clone().unwrap_or(vec![
                        vec![
                            None;
                            self.map_size.x as usize
                        ];
                        self.map_size.y as usize
                    ]),
                ),
            ))?;

        // Discards events while it doesn't find an Event::Moved or an Event::TileContentUpdated
        while let Some(event) = self.event_queue().borrow_mut().pop_front() {
            match event {
                Event::Moved(_tile, coords) => {
                    self.player_component
                        .update(PlayerComponentUpdateParam::new(coords))?;
                    self.set_center(gfx, vec2(coords.1 as f32, coords.0 as f32));
                    break;
                }
                Event::TileContentUpdated(tile, coords) => {
                    self.contents_map_component
                        .update(ContentsMapComponentUpdateParam::new(
                            ContentsMapComponentUpdateType::ContentChange(tile, coords),
                        ))?;
                    break;
                }
                _ => {}
            };
        }

        Ok(())
    }

    /// The function returns the reference to the origin of the visualizer.
    pub fn origin(&self) -> Vec2 {
        self.origin
    }

    /// The function returns the shared reference to the event_queue of the visualizer.
    pub fn event_queue(&self) -> Rc<RefCell<VecDeque<Event>>> {
        self.event_queue_rc.clone()
    }
}
