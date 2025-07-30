pub trait Interactable {
    #[allow(unused_variables)]
    fn handle_click(&mut self, position: [f32; 2]) {}
}
