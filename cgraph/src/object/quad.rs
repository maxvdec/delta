use crate::{
    app::Window,
    object::{Object, Vertex},
};
use glam::Vec4;

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

pub type Color = Vec4;

pub fn create_quad(
    window: &mut Window,
    size: Size,
    color: Color,
    z_index: f32,
    position: Position,
) -> () {
    let window_width = window.width as f32;
    let window_height = window.height as f32;

    let norm_x = position.x / window_width;
    let norm_y = position.y / window_height;
    let norm_width = size.width / window_width;
    let norm_height = size.height / window_height;

    let vertices = vec![
        Vertex::new(norm_x, norm_y, z_index, color), // Bottom-left
        Vertex::new(norm_x + norm_width, norm_y, z_index, color), // Bottom-right
        Vertex::new(norm_x + norm_width, norm_y + norm_height, z_index, color), // Top-right
        Vertex::new(norm_x, norm_y + norm_height, z_index, color), // Top-left
    ];

    let indices = vec![0, 1, 2, 0, 2, 3];
    let mut object = Object::new(vertices, indices);
    object.update_buffer();
    window.add_object(object);
}

pub fn create_rounded_quad(
    window: &mut Window,
    size: Size,
    color: Color,
    z_index: f32,
    position: Position,
) -> () {
    let window_width = window.width as f32;
    let window_height = window.height as f32;

    let norm_x = position.x / window_width;
    let norm_y = position.y / window_height;
    let norm_width = size.width / window_width;
    let norm_height = size.height / window_height;

    let vertices = vec![
        Vertex::new(norm_x, norm_y, z_index, color),
        Vertex::new(norm_x + norm_width, norm_y, z_index, color),
        Vertex::new(norm_x + norm_width, norm_y + norm_height, z_index, color),
        Vertex::new(norm_x, norm_y + norm_height, z_index, color),
    ];

    let indices = vec![0, 1, 2, 0, 2, 3];
    let mut object = Object::new(vertices, indices);
    object.update_buffer();
    window.add_object(object);
}
