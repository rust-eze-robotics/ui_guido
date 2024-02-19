use robotics_lib::{
    energy::Energy,
    event::events::Event,
    runner::{backpack::BackPack, Robot, Runnable},
    world::coordinates::Coordinate,
};
use rust_eze_spotlight::Spotlight;
use rust_eze_tomtom::TomTom;
use ui_lib::RunnableUi;

pub struct MyRobot {
    pub runnable_ui: Box<dyn RunnableUi>,
    pub robot: Robot,
}

impl MyRobot {
    pub fn new(runnable_ui: Box<dyn RunnableUi>, robot: Robot) -> Self {
        MyRobot { runnable_ui, robot }
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

        // Required world update to the visualizer. See ui_lib::RunnableUi
        // documentation for more informations.
        self.runnable_ui.process_tick(world);
    }

    fn handle_event(&mut self, event: Event) {
        self.runnable_ui.handle_event(event);
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
