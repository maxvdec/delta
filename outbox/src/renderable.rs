use crate::event::EventManager;

#[derive(Debug, Clone)]
pub enum PaddingDirection {
    Top,
    Bottom,
    Left,
    Right,
    Vertical,
    Horizontal,
}

pub trait Renderable {
    #[allow(unused_variables)]
    fn render(
        &self,
        canvas_size: [f32; 2],
        assigned_position: [f32; 2],
    ) -> Vec<cgraph::object::Object> {
        vec![]
    }

    fn get_size(&self) -> [f32; 2] {
        [0.0, 0.0]
    }
    fn get_padding(&self) -> [f32; 4] {
        [0.0, 0.0, 0.0, 0.0]
    }
    fn get_event_handler(&self) -> Option<&EventManager> {
        None
    }
    fn get_event_handler_mut(&mut self) -> Option<&mut EventManager> {
        None
    }
    fn copy(&self) -> Box<dyn Renderable>;
}
