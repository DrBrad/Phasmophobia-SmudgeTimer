use std::any::Any;
use std::collections::HashMap;
use rdev::Key;
use crate::bus::events::inter::event::Event;

#[derive(Debug, Clone)]
pub struct ButtonEvent {
    prevent_default: bool,
    pub(crate) button: Key
}

impl ButtonEvent {

    pub fn new(button: Key) -> Self {
        Self {
            prevent_default: false,
            button
        }
    }
}

impl Event for ButtonEvent {

    fn get_name(&self) -> String {
        String::from("button_event")
    }

    fn is_prevent_default(&self) -> bool {
        todo!()
    }

    fn prevent_default(&mut self) {
        todo!()
    }

    fn upcast(&self) -> &dyn Event {
        self
    }

    fn upcast_mut(&mut self) -> &mut dyn Event {
        self
    }

    fn dyn_clone(&self) -> Box<dyn Event> {
        Box::new(self.clone())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
