use cgraph::app::CoreEvent;
use std::sync::{Arc, Mutex};

pub type EventHandler = Arc<dyn Fn(&CoreEvent) + Send + Sync>;
pub type ClickHandler<T> = Arc<Mutex<dyn FnMut(&mut T) + Send + Sync>>;
pub type HoverHandler<T> = Arc<Mutex<dyn FnMut(&mut T, bool) + Send + Sync>>;

#[derive(Default)]
pub struct EventManager {
    events: Vec<EventHandler>,
    focused: bool,
    // Store bounds for hit testing [x, y, width, height]
    bounds: Option<[f32; 4]>,
}

impl Clone for EventManager {
    fn clone(&self) -> Self {
        Self {
            events: self.events.clone(),
            focused: self.focused,
            bounds: self.bounds,
        }
    }
}

impl EventManager {
    pub fn register_event(&mut self, handler: EventHandler) {
        self.events.push(handler);
    }

    pub fn handle_event(&self, event: &CoreEvent) {
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

    pub fn set_bounds(&mut self, x: f32, y: f32, width: f32, height: f32) {
        self.bounds = Some([x, y, width, height]);
    }

    pub fn get_bounds(&self) -> Option<[f32; 4]> {
        self.bounds
    }

    pub fn contains_point(&self, x: f32, y: f32) -> bool {
        if let Some([bx, by, width, height]) = self.bounds {
            x >= bx && x <= bx + width && y >= by && y <= by + height
        } else {
            false
        }
    }
}
