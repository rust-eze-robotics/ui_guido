use std::{rc::Rc, cell::RefCell, collections::VecDeque};

use robotics_lib::{event::events::Event, world::{tile::Tile, World}, interface::robot_map};
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
        UiWrapper {
            event_queue,
            world,
        }
    }    

    /// Returns the world.
    pub fn world(&self) -> Rc<RefCell<Option<Vec<Vec<Option<Tile>>>>>> {
        self.world.clone()
    }

    /// Returns the event queue.
    pub fn event_queue(&self) -> Rc<RefCell<VecDeque<Event>>> {
        self.event_queue.clone()
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
