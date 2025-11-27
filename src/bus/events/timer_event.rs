use std::any::Any;
use std::collections::HashMap;
use rdev::Key;
use crate::bus::events::inter::event::Event;

#[derive(Debug, Clone)]
pub struct TimerEvent {
    prevent_default: bool,
    pub(crate) time: u128
}

impl TimerEvent {

    pub fn new(time: u128) -> Self {
        Self {
            prevent_default: false,
            time
        }
    }
}

impl Event for TimerEvent {

    fn get_name(&self) -> String {
        String::from("timer_event")
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
