use crate::renderable::Renderable;

#[derive(Default)]
pub struct Empty {}

impl Renderable for Empty {
    fn render(
        &self,
        _canvas_size: [f32; 2],
        _assigned_position: [f32; 2],
    ) -> Vec<cgraph::object::Object> {
        Vec::new()
    }

    fn get_size(&self) -> [f32; 2] {
        [0.0, 0.0]
    }

    /// ## Warning
    /// Do not use this method, use a `Spacer` instead.
    fn padding(&mut self, _padding: [f32; 4]) -> &mut dyn Renderable {
        self
    }

    fn padding_at(
        &mut self,
        _direction: crate::renderable::PaddingDirection,
        _padding: f32,
    ) -> &mut dyn Renderable {
        self
    }
    fn padding_area(
        &mut self,
        _direction: crate::renderable::PaddingDirection,
        _padding: [f32; 2],
    ) -> &mut dyn Renderable {
        self
    }
    fn get_padding(&self) -> [f32; 4] {
        [0.0, 0.0, 0.0, 0.0]
    }
    fn copy(&mut self) -> Box<dyn Renderable> {
        Box::new(Empty::default())
    }
}
