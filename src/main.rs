use std::{cell::RefCell, env, path::PathBuf, rc::Rc};

use gamepad::GamePad;
use ggez::{
    event::{Axis, EventHandler},
    glam::vec2,
};
use midgard::world_generator::{WorldGenerator, WorldGeneratorParameters};
use robot::MyRobot;
use visualizer::Visualizer;

use robotics_lib::{
    energy::Energy,
    event::events::Event,
    interface::{go, robot_map, robot_view, Direction},
    runner::{backpack::BackPack, Robot, Runnable, Runner},
    world::{coordinates::Coordinate, tile::Tile, world_generator::Generator, World},
};

mod gamepad;
mod robot;
pub mod visualizer;

struct State {
    // map: Vec<Vec<Tile>>,
    map: Rc<RefCell<Vec<Vec<Tile>>>>,
    world: Rc<RefCell<Option<Vec<Vec<Option<Tile>>>>>>,
    // image_scale: f32,
    visualizer: Visualizer,
    gamepad: GamePad,
}

impl EventHandler for State {
    fn update(&mut self, ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        let screen_width = ctx.gfx.window().inner_size().width as f32;
        let screen_height = ctx.gfx.window().inner_size().height as f32;
        if ctx.time.ticks() % 100 == 0 {
            if self.visualizer.event_queue().borrow_mut().pop().is_some() {
                println!("Event detected");
                self.visualizer.handle_event();
            } else {
                self.visualizer.next_tick();
            }

            println!(
                "Robot: {:?}",
                self.visualizer.runner().get_robot().get_coordinate()
            );
            println!("Delta frame time: {:?} ", ctx.time.delta());
            println!("Average FPS: {}", ctx.time.fps());
            println!("Origin: {:?}", self.visualizer.origin());
            println!("Scale: {:?}", self.visualizer.scale());
            println!("Screen: {:?}", vec2(screen_width, screen_height));
        }
        self.visualizer
            .add_offset(self.gamepad.get_leftstick_offset());
        self.visualizer
            .add_scale(ctx, self.gamepad.get_rightstick_offset().y);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        self.visualizer.draw(ctx)?;
        Ok(())
    }
    //
    // fn mouse_wheel_event(&mut self, _ctx: &mut ggez::Context, _x: f32, _y: f32) -> Result<(), ggez::GameError> {
    //     if (_y > 0.0) {
    //         self.visualizer.increase_scale();
    //     } else {
    //         self.visualizer.decrease_scale();
    //     }
    //
    //     Ok(())
    // }
    //
    // fn key_down_event(
    //     &mut self,
    //     ctx: &mut ggez::Context,
    //     input: ggez::input::keyboard::KeyInput,
    //     _repeated: bool,
    // ) -> Result<(), ggez::GameError> {
    //
    //     Ok(())
    // }

    fn gamepad_axis_event(
        &mut self,
        _ctx: &mut ggez::Context,
        axis: ggez::input::gamepad::gilrs::Axis,
        value: f32,
        _id: ggez::event::GamepadId,
    ) -> Result<(), ggez::GameError> {
        if axis == Axis::LeftStickY {
            self.gamepad.set_leftstick_y_offset(value);
        } else if axis == Axis::LeftStickX {
            self.gamepad.set_leftstick_x_offset(value);
        } else if axis == Axis::RightStickY {
            self.gamepad.set_rightstick_y_offset(value);
        }
        Ok(())
    }
}

fn main() {
    let (ctx, event_loop) = ggez::ContextBuilder::new("robotics", "ggez")
        .window_setup(
            ggez::conf::WindowSetup::default()
                .title("Robotics")
                .vsync(true),
        )
        .window_mode(ggez::conf::WindowMode::default().dimensions(1600.0, 1200.0))
        .add_resource_path(match env::var("CARGO_MANIFEST_DIR") {
            Ok(manifest_dir) => {
                let mut path = PathBuf::from(manifest_dir);
                path.push("resources");
                path
            }
            Err(_) => PathBuf::from("./resources"),
        })
        .build()
        .unwrap();

    let params = WorldGeneratorParameters {
        world_size: 200,
        amount_of_rivers: Some(4.0),
        amount_of_streets: Some(3.0),
        amount_of_teleports: Some(2.0),
        always_sunny: true,
        ..Default::default()
    };

    let mut world_generator = WorldGenerator::new(params);
    let (map, spawn_point, _weather, _max_score, _score_table) = world_generator.gen();

    // let state = State {
    //     image: textures::TileTypeTexture::from_tiletype(
    //         &ctx,
    //         &robotics_lib::world::tile::TileType::Street,
    //     )
    //     .half,
    //     map_images: textures::load_tiles_matrix_textures(&ctx, &map),
    //     map,
    //     image_scale: 4.0
    // };
    //
    //
    //

    let mut world_rc: Rc<RefCell<Option<Vec<Vec<Option<Tile>>>>>> = Rc::new(RefCell::new(None));
    let mut event_queue: Rc<RefCell<Vec<Event>>> = Rc::new(RefCell::new(Vec::new()));

    let runner = Runner::new(
        Box::new(MyRobot::new(world_rc.clone(), event_queue.clone())),
        &mut world_generator,
    )
    .unwrap();

    let map_rc = Rc::new(RefCell::new(map));

    let mut state = State {
        world: world_rc.clone(),
        map: map_rc.clone(),
        visualizer: Visualizer::new(
            &ctx,
            world_rc.clone(),
            event_queue.clone(),
            runner,
            map_rc.clone(),
            vec2(0.0, 0.0),
            4.0,
        ),
        gamepad: GamePad::new(),
    };

    // state.visualizer.set_center(&ctx, vec2(0.0, 0.0));
    // state
    //     .visualizer
    //     .set_center(&ctx, vec2(0.0, (map.len() / 2) as f32));

    ggez::event::run(ctx, event_loop, state);
}
