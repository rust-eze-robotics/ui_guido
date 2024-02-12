use std::{rc::Rc, cell::RefCell};

use robotics_lib::{world::{tile::Tile, coordinates::Coordinate, World}, runner::{Runnable, backpack::BackPack, Robot}, interface::{robot_map, Direction, go}, event::events::Event, energy::Energy};
use rust_eze_tomtom::TomTom;
use rust_eze_spotlight::Spotlight;

pub struct MyRobot {
    pub world: Rc<RefCell<Option<Vec<Vec<Option<Tile>>>>>>,
    robot: Robot,
}

impl MyRobot {
    pub fn new(
        world: Rc<RefCell<Option<Vec<Vec<Option<Tile>>>>>>,
    ) -> Self {
        MyRobot {
            world,
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

