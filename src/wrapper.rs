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

pub struct RobotWrapper {
    pub world: Rc<RefCell<Option<Vec<Vec<Option<Tile>>>>>>,
    pub runnable: Box<dyn Runnable>,
}

impl RobotWrapper {
    pub fn new(
        world: Rc<RefCell<Option<Vec<Vec<Option<Tile>>>>>>,
        runnable: Box<dyn Runnable>,
    ) -> Self {
        RobotWrapper {
            world,
            runnable,
        }
    }

    pub fn world(&self) -> Rc<RefCell<Option<Vec<Vec<Option<Tile>>>>>> {
        self.world.clone()
    }
}

impl Runnable for RobotWrapper {
    fn process_tick(&mut self, world: &mut robotics_lib::world::World) {
        self.runnable.process_tick(world);
        self.world.replace(Some(robot_map(world).unwrap()));
    }

    fn handle_event(&mut self, event: Event) {
        self.runnable.handle_event(event.clone());
    }

    fn get_energy(&self) -> &Energy {
        self.runnable.get_energy()
    }
    fn get_energy_mut(&mut self) -> &mut Energy {
        self.runnable.get_energy_mut()
    }

    fn get_coordinate(&self) -> &Coordinate {
        self.runnable.get_coordinate()
    }
    fn get_coordinate_mut(&mut self) -> &mut Coordinate {
        self.runnable.get_coordinate_mut()
    }

    fn get_backpack(&self) -> &BackPack {
        self.runnable.get_backpack()
    }
    fn get_backpack_mut(&mut self) -> &mut BackPack {
        self.runnable.get_backpack_mut()
    }
}
