use std::{cell::RefCell, rc::Rc};

use robotics_lib::{
    energy::Energy,
    event::events::Event,
    interface::{go, robot_map, Direction},
    runner::{backpack::BackPack, Robot, Runnable},
    world::{coordinates::Coordinate, tile::Tile, World},
};
use rust_eze_spotlight::Spotlight;
use rust_eze_tomtom::TomTom;

pub struct MyRobot {
    pub world: Rc<RefCell<Option<Vec<Vec<Option<Tile>>>>>>,
    pub event_queue: Rc<RefCell<Vec<Event>>>,
    robot: Robot,
}

impl MyRobot {
    pub fn new(
        world: Rc<RefCell<Option<Vec<Vec<Option<Tile>>>>>>,
        event_queue: Rc<RefCell<Vec<Event>>>,
    ) -> Self {
        MyRobot {
            world,
            event_queue,
            robot: Robot::new(),
        }
    }

    pub fn world(&self) -> Rc<RefCell<Option<Vec<Vec<Option<Tile>>>>>> {
        self.world.clone()
    }
}

impl Runnable for MyRobot {
    fn process_tick(&mut self, world: &mut robotics_lib::world::World) {
        Spotlight::illuminate(self, world, 10);
        TomTom::go_to_tile(
            self,
            world,
            false,
            None,
            Some(rust_eze_tomtom::plain::PlainContent::Bush),
        );
        // go(self, world, Direction::Right);
        // go(self, world, Direction::Down);

        self.world.replace(Some(robot_map(world).unwrap()));
    }

    fn handle_event(&mut self, _event: Event) {
        self.event_queue.borrow_mut().push(_event);
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
