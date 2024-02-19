use std::{cell::RefCell, collections::VecDeque, env, path::PathBuf, rc::Rc};

use ai_builder::{get_world_generator_parameters as builder_get_world_generator_parameters, BuilderAi};
use gamepad::GamePad;
use ggez::{
    event::{Axis, EventHandler},
    glam::vec2,
    graphics::FontData,
};
use midgard::{params::{WorldGeneratorParameters, ContentsRadii}, WorldGenerator};
use robot::MyRobot;
use rusteze_ai_artemisia::{
    get_world_generator_parameters as artemis_get_world_generator_parameters, ArtemisIA,
};
use visualizer::Visualizer;

use robotics_lib::{
    event::events::Event,
    runner::{Robot, Runnable, Runner},
    world::{tile::Tile, world_generator::Generator},
};
use wrapper::UiWrapper;

mod gamepad;
mod robot;
mod visualizer;
mod wrapper;

const WORLD_SIZE: usize = 256;
const WORLD_SCALE: f64 = 0.5;

struct State {
    visualizer: Visualizer,
    gamepad: GamePad,
}

impl EventHandler for State {
    fn update(&mut self, ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        if ctx.time.ticks() % 50 == 0 {
            if self
                .visualizer
                .event_queue()
                .borrow_mut()
                .pop_front()
                .is_some()
            {
                self.visualizer.handle_event(&ctx.gfx)?;
            } else {
                if let Err(error) = self.visualizer.next_tick() {
                    return Err(ggez::GameError::CustomError(format!(
                        "Robot tick thrown the following error: {:?}",
                        error
                    )));
                }
            }
        }

        // Sends user input to the visualizer
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
    // Gets the robot name from the command line arguments.
    let robot_name = std::env::args()
        .map(|arg| arg.to_owned())
        .collect::<Vec<String>>()
        .get(1)
        .unwrap_or(&String::from("myrobot"))
        .clone();

    // Create a new context and event loop.
    let (mut ctx, event_loop) = ggez::ContextBuilder::new("ui_guido", "Davide Andreolli")
        .window_setup(
            ggez::conf::WindowSetup::default()
                .title("Guido - An alternative UI for runnable robotics")
                .vsync(true)
                .icon("/icon.png"),
        )
        .window_mode(
            ggez::conf::WindowMode::default()
                .dimensions(1600.0, 1200.0)
                .resizable(true)
                .transparent(true),
        )
        .add_resource_path(match env::var("CARGO_MANIFEST_DIR") {
            Ok(manifest_dir) => {
                let mut path = PathBuf::from(manifest_dir);
                path.push("resources");
                path
            }
            Err(_) => PathBuf::from("./resources"),
        })
        .build()
        .unwrap_or_else(|error| {
            panic!("Error while building the context: {:?}", error);
        });

    // Adds the font to the context.
    ctx.gfx.add_font(
        "kode",
        FontData::from_path(&ctx, "/fonts/kode.ttf").unwrap(),
    );

    // Creates parameters based on choosen robot.
    let world_generator_parameters = match robot_name.clone().as_str() {
        "myrobot" => WorldGeneratorParameters {
            world_size: WORLD_SIZE,
            world_scale: WORLD_SCALE,
            seed: 0,
            ..Default::default()
        },
        "artemisia" => WorldGeneratorParameters {
            world_size: WORLD_SIZE,
            world_scale: WORLD_SCALE,
            ..artemis_get_world_generator_parameters()
        },
        "builder" => WorldGeneratorParameters {
            world_size: WORLD_SIZE,
            world_scale: WORLD_SCALE,
            ..builder_get_world_generator_parameters()
        },        
        _ => panic!("Unknown robot name: {}", robot_name),
    };

    // Creates the world generator parameters.
    let params = WorldGeneratorParameters {
        world_size: WORLD_SIZE,
        world_scale: WORLD_SCALE,
        contents_radii: ContentsRadii {
            ..world_generator_parameters.contents_radii
        },
        ..world_generator_parameters
    };

    // Generates the world.
    let mut world_generator = WorldGenerator::new(params);
    let (map, spawn_point, _weather, _max_score, _score_table) = world_generator.gen();

    // Creates the shared states.
    let world_rc: Rc<RefCell<Option<Vec<Vec<Option<Tile>>>>>> = Rc::new(RefCell::new(None));
    let event_queue_rc: Rc<RefCell<VecDeque<Event>>> = Rc::new(RefCell::new(VecDeque::new()));
    let map_rc = Rc::new(RefCell::new(map));

    // Creates the UI wrapper for the robot.
    let runnable_ui = UiWrapper::new(event_queue_rc.clone(), world_rc.clone());

    // Chooses the runnable based on the robot name.
    let runnable: Box<dyn Runnable> = match robot_name.clone().as_str() {
        "myrobot" => Box::new(MyRobot::new(Box::new(runnable_ui), Robot::new())),
        "artemisia" => Box::new(ArtemisIA::new(WORLD_SIZE, Box::new(runnable_ui))),
        "builder" => Box::new(BuilderAi::new(Box::new(runnable_ui), WORLD_SIZE)),
        _ => panic!("Unknown robot name: {}", robot_name),
    };

    let runner = Runner::new(runnable, &mut world_generator).unwrap();

    // Creates the visualizer.
    let mut visualizer = Visualizer::new(
        &ctx,
        runner,
        world_rc.clone(),
        event_queue_rc.clone(),
        map_rc.clone(),
        spawn_point,
        4.0,
    );

    // Centers the visualizer on the spawn point at start.
    visualizer.set_center(&ctx.gfx, vec2(spawn_point.1 as f32, spawn_point.0 as f32));

    let state = State {
        visualizer,
        gamepad: GamePad::new(),
    };

    // Runs the event loop.
    ggez::event::run(ctx, event_loop, state);
}
