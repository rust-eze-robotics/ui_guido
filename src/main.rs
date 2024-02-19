use std::{cell::RefCell, collections::VecDeque, env, path::PathBuf, rc::Rc};

use gamepad::GamePad;
use ggez::{
    event::{Axis, EventHandler},
    graphics::FontData,
};
use midgard::world_generator::{WorldGenerator, WorldGeneratorParameters};
use robot::MyRobot;
use visualizer::Visualizer;

use robotics_lib::{
    event::events::Event,
    runner::{Robot, Runner},
    world::{tile::Tile, world_generator::Generator},
};
use wrapper::UiWrapper;

mod gamepad;
mod robot;
mod visualizer;
mod wrapper;

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

    ctx.gfx.add_font(
        "kode",
        FontData::from_path(&ctx, "/fonts/kode.ttf").unwrap(),
    );

    // Creates the world generator parameters.
    let params = WorldGeneratorParameters {
        // seed: 1,     // Uncomment to have a deterministic world.
        world_size: 30,
        amount_of_rivers: Some(4.0),
        amount_of_streets: Some(3.0),
        amount_of_teleports: Some(2.0),
        always_sunny: true,
        ..Default::default()
    };

    // Generates the world.
    let mut world_generator = WorldGenerator::new(params);
    let (map, spawn_point, _weather, _max_score, _score_table) = world_generator.gen();

    let world_rc: Rc<RefCell<Option<Vec<Vec<Option<Tile>>>>>> = Rc::new(RefCell::new(None));
    let event_queue_rc: Rc<RefCell<VecDeque<Event>>> = Rc::new(RefCell::new(VecDeque::new()));
    let map_rc = Rc::new(RefCell::new(map));

    // Creates the robot and the wrapper.
    let robot = Robot::new();
    let runnable_ui = UiWrapper::new(event_queue_rc.clone(), world_rc.clone());

    let my_robot = MyRobot::new(Box::new(runnable_ui), robot);
    let runner = Runner::new(Box::new(my_robot), &mut world_generator).unwrap();

    // Creates the visualizer.
    let visualizer = Visualizer::new(
        &ctx,
        runner,
        world_rc.clone(),
        event_queue_rc.clone(),
        map_rc.clone(),
        spawn_point,
        4.0,
    );

    let state = State {
        visualizer,
        gamepad: GamePad::new(),
    };

    // Runs the event loop.
    ggez::event::run(ctx, event_loop, state);
}
