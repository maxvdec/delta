use crate::{
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
    size: Size,
    color: Color,
    z_index: f32,
    position: Position,
) -> Object {
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
    object
}

pub fn create_rounded_quad(
    size: Size,
    color: Color,
    z_index: f32,
    position: Position,
    corner_radius: f32,
) -> Object {
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
    object.corner_radius = corner_radius;
    object.update_buffer();
    object
}

pub fn create_circle(
    size: Size,
    color: Color,
    z_index: f32,
    position: Position,
) -> Object {
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
    object.rotation = 0.0;
    object.corner_radius = half_height; // Use half height as corner radius for a circle
    object.update_buffer();
    object
}

pub fn create_circle_with_radius(
    radius: f32,
    color: Color,
    z_index: f32,
    position: Position,
) -> Object {
    let size = Size::new(radius * 2.0, radius * 2.0);
    create_circle(size, color, z_index, position)
}

pub fn create_polygon(
    size: Size,
    color: Color,
    z_index: f32,
    position: Position,
    faces: usize,
) -> Object {
    if faces < 3 {
        // Return a default empty object for invalid polygon
        return Object::new(vec![], vec![]);
    }

    let radius = size.width.min(size.height) / 2.0;
    let center_x = position.x;
    let center_y = position.y;

    let mut vertices = vec![Vertex::new(
        center_x,
        center_y,
        z_index,
        color,
        Vec2::new(0.5, 0.5),
    )];

    for i in 0..faces {
        let angle = 2.0 * std::f32::consts::PI * i as f32 / faces as f32;
        let x = center_x + radius * angle.cos();
        let y = center_y + radius * angle.sin();

        let u = (x - (center_x - radius)) / (2.0 * radius);
        let v = (y - (center_y - radius)) / (2.0 * radius);

        vertices.push(Vertex::new(x, y, z_index, color, Vec2::new(u, v)));
    }

    let mut indices: Vec<u32> = Vec::new();
    for i in 0..faces {
        let next = if i == faces - 1 { 1 } else { (i + 2) as u32 };
        indices.extend_from_slice(&[0, (i + 1) as u32, next]);
    }

    let mut object = Object::new(vertices, indices);
    object.position = Vec2::new(position.x, position.y);
    object.scale = Vec2::new(1.0, 1.0);
    object.original_pixel_size = Vec2::new(size.width, size.height);
    object.rotation = 0.0;
    object.corner_radius = 0.0;
    object.update_buffer();
    object
}

// Textured object creation functions

#[cfg(target_os = "macos")]
pub fn create_textured_quad(
    size: Size,
    z_index: f32,
    position: Position,
    image_path: &str,
) -> Result<Object, Box<dyn std::error::Error>> {
    use crate::macos::image::Image;
    
    let image = Image::new(image_path)?;
    let mut object = create_quad(size, Vec4::new(1.0, 1.0, 1.0, 1.0), z_index, position);
    object = object.with_texture(image);
    Ok(object)
}

#[cfg(target_os = "macos")]
pub fn create_textured_quad_with_device(
    size: Size,
    z_index: f32,
    position: Position,
    image_path: &str,
    device: &metal::Device,
) -> Result<Object, Box<dyn std::error::Error>> {
    use crate::macos::image::Image;
    
    let image = Image::new_from_device(image_path, device)?;
    let mut object = create_quad(size, Vec4::new(1.0, 1.0, 1.0, 1.0), z_index, position);
    object = object.with_texture(image);
    Ok(object)
}

#[cfg(target_os = "macos")]
pub fn create_textured_rounded_quad(
    size: Size,
    z_index: f32,
    position: Position,
    corner_radius: f32,
    image_path: &str,
) -> Result<Object, Box<dyn std::error::Error>> {
    use crate::macos::image::Image;
    
    let image = Image::new(image_path)?;
    let mut object = create_rounded_quad(size, Vec4::new(1.0, 1.0, 1.0, 1.0), z_index, position, corner_radius);
    object = object.with_texture(image);
    Ok(object)
}
