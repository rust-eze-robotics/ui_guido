use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use robotics_lib::{
    energy::Energy,
    event::events::Event,
    interface::robot_map,
    runner::{backpack::BackPack, Robot, Runnable},
    world::{coordinates::Coordinate, tile::Tile},
};
use rust_eze_spotlight::Spotlight;
use rust_eze_tomtom::TomTom;

pub struct MyRobot {
    pub world: Rc<RefCell<Option<Vec<Vec<Option<Tile>>>>>>,
    pub event_queue: Rc<RefCell<VecDeque<Event>>>,
    robot: Robot,
}

impl MyRobot {
    pub fn new(
        world: Rc<RefCell<Option<Vec<Vec<Option<Tile>>>>>>,
        event_queue: Rc<RefCell<VecDeque<Event>>>,
        robot: Robot,
    ) -> Self {
        MyRobot {
            world,
            event_queue,
            robot,
        }
    }
}

impl Runnable for MyRobot {
    fn process_tick(&mut self, world: &mut robotics_lib::world::World) {
        Spotlight::illuminate(self, world, 10).unwrap_or_else(|error| {
            println!("Spotlight error: {:?}", error);
        });

        if let Err(error) = TomTom::go_to_tile(
            self,
            world,
            false,
            None,
            Some(rust_eze_tomtom::plain::PlainContent::Bush),
        ) {
            println!("TomTom error: {:?}", error);
        }

        // Required world update to the visualizer. See the RunnableWrapper
        // documentation for more information.
        self.world.replace(Some(robot_map(world).unwrap()));
    }

    fn handle_event(&mut self, event: Event) {
        self.event_queue.borrow_mut().push_back(event);
    }

    fn get_energy(&self) -> &Energy {
        &self.robot.energy
    }

    fn get_energy_mut(&mut self) -> &mut Energy {
        &mut self.robot.energy
    }

    fn get_coordinate(&self) -> &Coordinate {
        &self.robot.coordinate
    }

    fn get_coordinate_mut(&mut self) -> &mut Coordinate {
        &mut self.robot.coordinate
    }

    fn get_backpack(&self) -> &BackPack {
        &self.robot.backpack
    }

    fn get_backpack_mut(&mut self) -> &mut BackPack {
        &mut self.robot.backpack
    }
}
