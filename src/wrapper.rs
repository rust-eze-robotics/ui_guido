use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use robotics_lib::{
    event::events::Event,
    interface::robot_map,
    world::{tile::Tile, World},
};
use ui_lib::RunnableUi;

/// UiWrapper is used by Runnable to communicate world updates and
/// events to the visualizer.
pub struct UiWrapper {
    event_queue: Rc<RefCell<VecDeque<Event>>>,
    world: Rc<RefCell<Option<Vec<Vec<Option<Tile>>>>>>,
}

impl UiWrapper {
    /// Creates a new instance of UiWrapper.
    pub fn new(
        event_queue: Rc<RefCell<VecDeque<Event>>>,
        world: Rc<RefCell<Option<Vec<Vec<Option<Tile>>>>>>,
    ) -> Self {
        UiWrapper { event_queue, world }
    }
}

impl RunnableUi for UiWrapper {
    /// Updates the world.
    fn process_tick(&mut self, world: &mut World) {
        self.world.replace(Some(robot_map(world).unwrap()));
    }

    /// Pushes the coming event to the events queue.
    fn handle_event(&mut self, event: Event) {
        self.event_queue.borrow_mut().push_back(event);
    }
}
