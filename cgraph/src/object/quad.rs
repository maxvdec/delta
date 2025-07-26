use crate::{
    app::Window,
    object::{Object, Vertex},
};
use glam::{Vec2, Vec4};

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
    let half_width = size.width / 2.0;
    let half_height = size.height / 2.0;

    let left = position.x - half_width;
    let right = position.x + half_width;
    let bottom = position.y - half_height;
    let top = position.y + half_height;

    let vertices = vec![
        Vertex::new(left, bottom, z_index, color, Vec2::new(0.0, 0.0)), // Bottom-left (0)
        Vertex::new(right, bottom, z_index, color, Vec2::new(1.0, 0.0)), // Bottom-right (1)
        Vertex::new(right, top, z_index, color, Vec2::new(1.0, 1.0)),   // Top-right (2)
        Vertex::new(left, top, z_index, color, Vec2::new(0.0, 1.0)),    // Top-left (3)
    ];

    let indices = vec![
        0, 1, 2, // First triangle: bottom-left, bottom-right, top-right
        0, 2, 3, // Second triangle: bottom-left, top-right, top-left
    ];

    let mut object = Object::new(vertices, indices);
    object.position = Vec2::new(position.x, position.y);
    object.scale = Vec2::new(1.0, 1.0);
    object.original_pixel_size = Vec2::new(size.width, size.height);
    object.rotation = 0.0; // No rotation for a quad
    object.corner_radius = 0.0; // No corner radius for a simple quad
    object.update_buffer();
    window.add_object(object);
}

pub fn create_rounded_quad(
    window: &mut Window,
    size: Size,
    color: Color,
    z_index: f32,
    position: Position,
    corner_radius: f32,
) -> () {
    let half_width = size.width / 2.0;
    let half_height = size.height / 2.0;

    let left = position.x - half_width;
    let right = position.x + half_width;
    let bottom = position.y - half_height;
    let top = position.y + half_height;

    let vertices = vec![
        Vertex::new(left, bottom, z_index, color, Vec2::new(0.0, 0.0)), // Bottom-left
        Vertex::new(right, bottom, z_index, color, Vec2::new(1.0, 0.0)), // Bottom-right
        Vertex::new(right, top, z_index, color, Vec2::new(1.0, 1.0)),   // Top-right
        Vertex::new(left, top, z_index, color, Vec2::new(0.0, 1.0)),    // Top-left
    ];

    let indices = vec![
        0, 1, 2, // First triangle
        0, 2, 3, // Second triangle
    ];

    let mut object = Object::new(vertices, indices);
    object.position = Vec2::new(position.x, position.y);
    object.scale = Vec2::new(1.0, 1.0);
    object.original_pixel_size = Vec2::new(size.width, size.height);
    object.rotation = 0.0; // No rotation for a quad
    object.corner_radius = corner_radius; // No corner radius for a simple quad
    object.update_buffer();
    window.add_object(object);
}
