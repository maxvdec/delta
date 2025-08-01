use crate::object::primitives::{Color, Position};
use crate::object::{
    Object,
    curve::{Curve, Path, Point},
};

/// Creates a quadratic Bezier curve object.
pub fn create_quadratic_bezier(
    start: Position,
    control: Position,
    end: Position,
    color: Color,
    z_index: f32,
    line_width: f32,
    steps: usize,
) -> Object {
    let curve = Curve::Quadratic {
        p0: Point {
            x: start.x,
            y: start.y,
        },
        p1: Point {
            x: control.x,
            y: control.y,
        },
        p2: Point { x: end.x, y: end.y },
    };

    let path = Path {
        curves: vec![curve],
    };
    path.to_object(steps, color, z_index, line_width)
}

/// Represents a control path for cubic Bezier curves.
pub struct ControlPath {
    /// The starting point of the cubic Bezier curve.
    pub start: Position,
    /// The first control point of the cubic Bezier curve.
    pub control1: Position,
    /// The second control point of the cubic Bezier curve.
    pub control2: Position,
    /// The ending point of the cubic Bezier curve.
    pub end: Position,
}

/// Creates a cubic Bezier curve object.
pub fn create_cubic_bezier(
    control: ControlPath,
    color: Color,
    z_index: f32,
    line_width: f32,
    steps: usize,
) -> Object {
    let curve = Curve::Cubic {
        p0: Point {
            x: control.start.x,
            y: control.start.y,
        },
        p1: Point {
            x: control.control1.x,
            y: control.control1.y,
        },
        p2: Point {
            x: control.control2.x,
            y: control.control2.y,
        },
        p3: Point {
            x: control.end.x,
            y: control.end.y,
        },
    };

    let path = Path {
        curves: vec![curve],
    };
    path.to_object(steps, color, z_index, line_width)
}

/// Represents an arc defined by its center, radius, and angles.
pub struct ArcAngle {
    /// The center of the arc.
    pub center: Position,
    /// The radius of the arc.
    pub radius: f32,
    /// The starting angle of the arc in radians.
    pub start_angle: f32,
    /// The ending angle of the arc in radians.
    pub end_angle: f32,
}

/// Creates an arc object.
pub fn create_arc(
    arc: ArcAngle,
    color: Color,
    z_index: f32,
    line_width: f32,
    steps: usize,
) -> Object {
    let curve = Curve::Arc {
        center: Point {
            x: arc.center.x,
            y: arc.center.y,
        },
        radius: arc.radius,
        start_angle: arc.start_angle,
        end_angle: arc.end_angle,
    };

    let path = Path {
        curves: vec![curve],
    };
    path.to_object(steps, color, z_index, line_width)
}

/// Creates a full circle arc object.
pub fn create_circle_arc(
    center: Position,
    radius: f32,
    color: Color,
    z_index: f32,
    line_width: f32,
    steps: usize,
) -> Object {
    create_arc(
        ArcAngle {
            center,
            radius,
            start_angle: 0.0,
            end_angle: 2.0 * std::f32::consts::PI,
        },
        color,
        z_index,
        line_width,
        steps,
    )
}

/// Creates a Catmull-Rom spline object.
pub fn create_catmull_rom_spline(
    points: Vec<Position>,
    tension: f32,
    color: Color,
    z_index: f32,
    line_width: f32,
    steps: usize,
) -> Object {
    if points.len() < 4 {
        // Not enough points for Catmull-Rom, return empty object
        return Object::new(vec![], vec![]);
    }

    let curve_points: Vec<Point> = points
        .iter()
        .map(|pos| Point { x: pos.x, y: pos.y })
        .collect();

    let curve = Curve::CatmullRom {
        points: curve_points,
        tension,
    };

    let path = Path {
        curves: vec![curve],
    };
    path.to_object(steps, color, z_index, line_width)
}

/// Creates a smooth path object from a series of points.
pub fn create_smooth_path(
    points: Vec<Position>,
    color: Color,
    z_index: f32,
    line_width: f32,
    steps: usize,
) -> Object {
    if points.len() < 2 {
        return Object::new(vec![], vec![]);
    }

    let mut curves = Vec::new();

    for i in 0..points.len() - 1 {
        let start = Point {
            x: points[i].x,
            y: points[i].y,
        };
        let end = Point {
            x: points[i + 1].x,
            y: points[i + 1].y,
        };

        // Create a simple control point at the midpoint for smoothness
        let control = Point {
            x: (start.x + end.x) / 2.0,
            y: (start.y + end.y) / 2.0,
        };

        let curve = Curve::Quadratic {
            p0: start,
            p1: control,
            p2: end,
        };

        curves.push(curve);
    }

    let path = Path { curves };
    path.to_object(steps, color, z_index, line_width)
}

/// Creates a path object from a series of points.
pub fn create_path_from_points(
    points: Vec<Position>,
    color: Color,
    z_index: f32,
    line_width: f32,
    steps: usize,
) -> Object {
    if points.len() < 2 {
        return Object::new(vec![], vec![]);
    }

    let mut curves = Vec::new();

    // Create linear segments between consecutive points using quadratic curves with control point at midpoint
    for i in 0..points.len() - 1 {
        let start = Point {
            x: points[i].x,
            y: points[i].y,
        };
        let end = Point {
            x: points[i + 1].x,
            y: points[i + 1].y,
        };

        // For linear segments, place control point exactly at midpoint
        let control = Point {
            x: (start.x + end.x) / 2.0,
            y: (start.y + end.y) / 2.0,
        };

        let curve = Curve::Quadratic {
            p0: start,
            p1: control,
            p2: end,
        };

        curves.push(curve);
    }

    let path = Path { curves };
    path.to_object(steps, color, z_index, line_width)
}

/// Represents a point in 2D space.
pub struct PathBuilder {
    curves: Vec<Curve>,
    current_point: Option<Point>,
}

/// Builds a path using a series of curves.
impl PathBuilder {
    /// Creates a new PathBuilder instance.
    pub fn new() -> Self {
        PathBuilder {
            curves: Vec::new(),
            current_point: None,
        }
    }

    /// Moves the current point to a new position.
    pub fn move_to(mut self, position: Position) -> Self {
        self.current_point = Some(Point {
            x: position.x,
            y: position.y,
        });
        self
    }

    /// Adds a line to the current point.
    pub fn line_to(mut self, position: Position) -> Self {
        if let Some(start) = self.current_point {
            let end = Point {
                x: position.x,
                y: position.y,
            };
            let control = Point {
                x: (start.x + end.x) / 2.0,
                y: (start.y + end.y) / 2.0,
            };

            let curve = Curve::Quadratic {
                p0: start,
                p1: control,
                p2: end,
            };

            self.curves.push(curve);
            self.current_point = Some(end);
        }
        self
    }

    /// Adds a quadratic Bezier curve to the path.
    pub fn quadratic_to(mut self, control: Position, end: Position) -> Self {
        if let Some(start) = self.current_point {
            let curve = Curve::Quadratic {
                p0: start,
                p1: Point {
                    x: control.x,
                    y: control.y,
                },
                p2: Point { x: end.x, y: end.y },
            };

            self.curves.push(curve);
            self.current_point = Some(Point { x: end.x, y: end.y });
        }
        self
    }

    /// Adds a cubic Bezier curve to the path.
    pub fn cubic_to(mut self, control1: Position, control2: Position, end: Position) -> Self {
        if let Some(start) = self.current_point {
            let curve = Curve::Cubic {
                p0: start,
                p1: Point {
                    x: control1.x,
                    y: control1.y,
                },
                p2: Point {
                    x: control2.x,
                    y: control2.y,
                },
                p3: Point { x: end.x, y: end.y },
            };

            self.curves.push(curve);
            self.current_point = Some(Point { x: end.x, y: end.y });
        }
        self
    }

    /// Adds an arc to the path.
    pub fn arc_to(
        mut self,
        center: Position,
        radius: f32,
        start_angle: f32,
        end_angle: f32,
    ) -> Self {
        let curve = Curve::Arc {
            center: Point {
                x: center.x,
                y: center.y,
            },
            radius,
            start_angle,
            end_angle,
        };

        self.curves.push(curve);

        // Update current point to the end of the arc
        let end_x = center.x + radius * end_angle.cos();
        let end_y = center.y + radius * end_angle.sin();
        self.current_point = Some(Point { x: end_x, y: end_y });
        self
    }

    /// Builds the path into an Object with specified color, z_index, line_width, and steps.
    pub fn build(self, color: Color, z_index: f32, line_width: f32, steps: usize) -> Object {
        let path = Path {
            curves: self.curves,
        };
        path.to_object(steps, color, z_index, line_width)
    }

    /// Builds the path into a Path object.
    pub fn build_path(self) -> Path {
        Path {
            curves: self.curves,
        }
    }
}

impl Default for PathBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Creates a heart shape object.
pub fn create_heart_shape(
    center: Position,
    size: f32,
    color: Color,
    z_index: f32,
    line_width: f32,
    steps: usize,
) -> Object {
    let scale = size / 100.0; // Normalize to size

    PathBuilder::new()
        .move_to(Position::new(center.x, center.y + 25.0 * scale))
        .cubic_to(
            Position::new(center.x, center.y + 5.0 * scale),
            Position::new(center.x - 25.0 * scale, center.y - 20.0 * scale),
            Position::new(center.x - 25.0 * scale, center.y - 35.0 * scale),
        )
        .cubic_to(
            Position::new(center.x - 25.0 * scale, center.y - 50.0 * scale),
            Position::new(center.x, center.y - 50.0 * scale),
            Position::new(center.x, center.y - 35.0 * scale),
        )
        .cubic_to(
            Position::new(center.x, center.y - 50.0 * scale),
            Position::new(center.x + 25.0 * scale, center.y - 50.0 * scale),
            Position::new(center.x + 25.0 * scale, center.y - 35.0 * scale),
        )
        .cubic_to(
            Position::new(center.x + 25.0 * scale, center.y - 20.0 * scale),
            Position::new(center.x, center.y + 5.0 * scale),
            Position::new(center.x, center.y + 25.0 * scale),
        )
        .build(color, z_index, line_width, steps)
}

/// Represents a star shape with its center, outer radius, inner radius, and number of points.
pub struct StarShape {
    /// The center of the star.
    pub center: Position,
    /// The outer radius of the star.
    pub outer_radius: f32,
    /// The inner radius of the star.
    pub inner_radius: f32,
    /// The number of points in the star.
    pub points: usize,
}

/// Creates a star shape object.
pub fn create_star_shape(
    shape: StarShape,
    color: Color,
    z_index: f32,
    line_width: f32,
    steps: usize,
) -> Object {
    if shape.points < 3 {
        return Object::new(vec![], vec![]);
    }

    let mut builder = PathBuilder::new();
    let angle_step = std::f32::consts::PI / shape.points as f32;

    // Start at the first outer point
    let start_angle = -std::f32::consts::PI / 2.0; // Start at top
    let start_x = shape.center.x + shape.outer_radius * start_angle.cos();
    let start_y = shape.center.y + shape.outer_radius * start_angle.sin();
    builder = builder.move_to(Position::new(start_x, start_y));

    for i in 0..shape.points * 2 {
        let angle = start_angle + (i as f32 * angle_step);
        let radius = if i % 2 == 0 {
            shape.inner_radius
        } else {
            shape.outer_radius
        };
        let x = shape.center.x + radius * angle.cos();
        let y = shape.center.y + radius * angle.sin();
        builder = builder.line_to(Position::new(x, y));
    }

    builder = builder.line_to(Position::new(start_x, start_y));

    builder.build(color, z_index, line_width, steps)
}
