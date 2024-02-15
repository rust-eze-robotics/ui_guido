use std::{cell::RefCell, rc::Rc};

use robotics_lib::{
    energy::Energy,
    event::events::Event,
    interface::robot_map,
    runner::{backpack::BackPack, Runnable},
    world::{coordinates::Coordinate, tile::Tile},
};

/// The structure implements the Runnable trait and works as intermediary
/// between the robot given and the visualizer. It is used to update the
/// visualizer's world state and to process the robot's actions.
///
/// Visualizer requires that the given Runner has internally a RunnableWrapper
/// instance as the robot.
/// Also, user robot, given for parameter 'runnable', MUST push events to the
/// given event queue.
///
/// If the previous rules are not followed, the visualizer will not work as
/// expected.
///
/// These restrictions are a limitation imposed by the robotic library,
/// unfortunately.
pub struct RunnableWrapper {
    world: Rc<RefCell<Option<Vec<Vec<Option<Tile>>>>>>,
    runnable: Box<dyn Runnable>,
}

impl RunnableWrapper {
    /// The constructor creates a new robot wrapper structure.
    pub fn new(
        world: Rc<RefCell<Option<Vec<Vec<Option<Tile>>>>>>,
        runnable: Box<dyn Runnable>,
    ) -> Self {
        RunnableWrapper { world, runnable }
    }
}

impl Runnable for RunnableWrapper {
    fn process_tick(&mut self, world: &mut robotics_lib::world::World) {
        // Process the robot's action and replace the shared world reference
        // content with the latest one.
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
