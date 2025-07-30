use cgraph::app::CoreEvent;
use std::sync::Arc;

pub type EventHandler = Arc<dyn Fn(&CoreEvent) + Send + Sync>;

#[derive(Default)]
pub struct EventManager {
    events: Vec<EventHandler>,
    focused: bool,
}

impl Clone for EventManager {
    fn clone(&self) -> Self {
        Self {
            events: self.events.clone(),
            focused: self.focused,
        }
    }
}

impl EventManager {
    pub fn register_event(&mut self, handler: EventHandler) {
        self.events.push(handler);
    }

    pub fn handle_event(&mut self, event: &CoreEvent) {
        for handler in &self.events {
            handler(event);
        }
    }

    pub fn set_focus(&mut self, focused: bool) {
        self.focused = focused;
    }

    pub fn is_focused(&self) -> bool {
        self.focused
    }
}
