use crate::{
    app::Window,
    object::{Object, Vertex},
};

pub struct Size {
    pub width: f32,
    pub height: f32,
}

impl Size {
    pub fn x(&self) -> f32 {
        self.width
    }

    pub fn y(&self) -> f32 {
        self.height
    }

    pub fn new(width: f32, height: f32) -> Self {
        Size { width, height }
    }
}

pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn new(x: f32, y: f32) -> Self {
        Position { x, y }
    }
}

pub type Color = [f32; 4];

pub fn create_quad(
    window: &mut Window,
    size: Size,
    color: Color,
    z_index: f32,
    position: Position,
) -> () {
    let vertices = vec![
        Vertex::new(position.x, position.y, z_index, color),
        Vertex::new(position.x + size.width, position.y, z_index, color),
        Vertex::new(
            position.x + size.width,
            position.y + size.height,
            z_index,
            color,
        ),
        Vertex::new(position.x, position.y + size.height, z_index, color),
    ];

    let indices = vec![0, 1, 2, 0, 2, 3];

    let mut object = Object::new(vertices, indices);

    object.update_buffer();

    window.add_object(object);
}
