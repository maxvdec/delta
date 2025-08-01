use crate::object::{Object, Vertex};
use glam::{Vec2, Vec4};

/// Represents a size with width and height.
pub struct Size {
    /// The width of the size.
    pub width: f32,
    /// The height of the size.
    pub height: f32,
}

impl Size {
    /// Returns the width of the size.
    pub fn x(&self) -> f32 {
        self.width
    }
    /// Returns the height of the size.
    pub fn y(&self) -> f32 {
        self.height
    }
    /// Creates a new size with the given width and height.
    pub fn new(width: f32, height: f32) -> Self {
        Size { width, height }
    }
}

/// Represents a position with x and y coordinates.
pub struct Position {
    /// The x coordinate of the position.
    pub x: f32,
    /// The y coordinate of the position.
    pub y: f32,
}

impl Position {
    /// Returns the x coordinate of the position.
    pub fn x(&self) -> f32 {
        self.x
    }
    /// Returns the y coordinate of the position.
    pub fn y(&self) -> f32 {
        self.y
    }
    /// Creates a new position with the given x and y coordinates.
    pub fn new(x: f32, y: f32) -> Self {
        Position { x, y }
    }
}

/// Represents a color with red, green, blue, and alpha components.
pub type Color = Vec4;

/// Creates a new `Object` representing a quad with the given size, color, z-index, and position.
pub fn create_quad(size: Size, color: Color, z_index: f32, position: Position) -> Object {
    let left = position.x;
    let right = if cfg!(target_os = "macos") {
        position.x + size.width * 2.0
    } else {
        position.x + size.width
    };
    let bottom = if cfg!(target_os = "macos") {
        position.y + size.height * 2.0
    } else {
        position.y + size.height
    };
    let top = position.y;

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

/// Creates a new `Object` representing a rounded quad with the given size, color, z-index, position, and corner radius.
pub fn create_rounded_quad(
    size: Size,
    color: Color,
    z_index: f32,
    position: Position,
    corner_radius: f32,
) -> Object {
    let left = position.x;
    let right = if cfg!(target_os = "macos") {
        position.x + size.width * 2.0
    } else {
        position.x + size.width
    };
    let bottom = if cfg!(target_os = "macos") {
        position.y + size.height * 2.0
    } else {
        position.y + size.height
    };
    let top = position.y;

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

/// Creates a new `Object` representing a circle with the given size, color, z-index, and position.
pub fn create_circle(size: Size, color: Color, z_index: f32, position: Position) -> Object {
    let left = position.x;
    let right = if cfg!(target_os = "macos") {
        position.x + size.width * 2.0
    } else {
        position.x + size.width
    };
    let bottom = if cfg!(target_os = "macos") {
        position.y + size.height * 2.0
    } else {
        position.y + size.height
    };
    let top = position.y;

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
    object.corner_radius = size.height.min(size.width) / 2.0; // Use half of the smaller dimension for circle
    object.update_buffer();
    object
}

/// Creates a new `Object` representing a circle with the given radius, color, z-index, and position.
pub fn create_circle_with_radius(
    radius: f32,
    color: Color,
    z_index: f32,
    position: Position,
) -> Object {
    let size = Size::new(radius * 2.0, radius * 2.0);
    create_circle(size, color, z_index, position)
}

/// Creates a new `Object` representing a polygon with the given size, color, z-index, position, and number of faces.
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
    // Calculate center based on top-left position
    let center_x = position.x + size.width / 2.0;
    let center_y = position.y + size.height / 2.0;

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

        // Calculate texture coordinates relative to the bounding box
        let u = (x - position.x) / size.width;
        let v = (y - position.y) / size.height;

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
/// Creates a new `Object` representing a textured quad with the given size, z-index, position, and image path.
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
/// Creates a new `Object` representing a textured rounded quad with the given size, z-index, position, corner radius, and image path.
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
/// Creates a new `Object` representing a textured rounded quad with the given size, z-index, position, corner radius, and image path.
pub fn create_textured_rounded_quad(
    size: Size,
    z_index: f32,
    position: Position,
    corner_radius: f32,
    image_path: &str,
) -> Result<Object, Box<dyn std::error::Error>> {
    use crate::macos::image::Image;

    let image = Image::new(image_path)?;
    let mut object = create_rounded_quad(
        size,
        Vec4::new(1.0, 1.0, 1.0, 1.0),
        z_index,
        position,
        corner_radius,
    );
    object = object.with_texture(image);
    Ok(object)
}

// Shadow helper functions
/// Creates a new `Object` representing a quad with shadow properties.
pub fn create_quad_with_shadow(
    size: Size,
    color: Color,
    z_index: f32,
    position: Position,
    shadow_radius: f32,
    shadow_color: Color,
    shadow_offset: Vec2,
) -> Object {
    let mut object = create_quad(size, color, z_index, position);
    object.set_shadow(shadow_radius, shadow_color, shadow_offset);
    object
}

/// Creates a new `Object` representing a rounded quad with shadow properties.
pub struct ShadowData {
    /// The radius of the shadow.
    pub radius: f32,
    /// The color of the shadow.
    pub color: Color,
    /// The offset of the shadow.
    pub offset: Vec2,
}

/// Creates a new `Object` representing a rounded quad with shadow properties.
pub fn create_rounded_quad_with_shadow(
    size: Size,
    color: Color,
    z_index: f32,
    position: Position,
    corner_radius: f32,
    shadow_data: ShadowData,
) -> Object {
    let mut object = create_rounded_quad(size, color, z_index, position, corner_radius);
    object.set_shadow(shadow_data.radius, shadow_data.color, shadow_data.offset);
    object
}

/// Creates a new `Object` representing a circle with shadow properties.
pub fn create_circle_with_shadow(
    size: Size,
    color: Color,
    z_index: f32,
    position: Position,
    shadow_data: ShadowData,
) -> Object {
    let mut object = create_circle(size, color, z_index, position);
    object.set_shadow(shadow_data.radius, shadow_data.color, shadow_data.offset);
    object
}
