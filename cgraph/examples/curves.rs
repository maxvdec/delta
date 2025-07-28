use cgraph::{
    app::Window,
    object::{
        curve_primitives::*,
        primitives::{Color, Position},
    },
};

fn main() {
    let mut win = Window::new("Curve Examples", 800, 600, None);

    // Create a quadratic Bézier curve
    win.add_object(create_quadratic_bezier(
        Position::new(100.0, 500.0),    // start
        Position::new(200.0, 200.0),    // control
        Position::new(300.0, 500.0),    // end
        Color::new(1.0, 0.0, 0.0, 1.0), // red
        1.0,                            // z_index
        3.0,                            // line_width
        50,                             // steps
    ));

    win.add_object(create_cubic_bezier(
        ControlPath {
            start: Position::new(350.0, 500.0),    // start
            control1: Position::new(400.0, 200.0), // control1
            control2: Position::new(500.0, 200.0), // control2
            end: Position::new(550.0, 500.0),
        }, // end
        Color::new(0.0, 1.0, 0.0, 1.0), // green
        1.0,                            // z_index
        3.0,                            // line_width
        50,                             // steps
    ));

    win.add_object(create_arc(
        ArcAngle {
            center: Position::new(400.0, 300.0), // center
            radius: 80.0,                        // radius
            start_angle: 0.0,                    // start_angle
            end_angle: std::f32::consts::PI,     // end_angle (half circle)
        },
        Color::new(0.0, 0.0, 1.0, 1.0), // blue
        1.0,                            // z_index
        3.0,                            // line_width
        50,                             // steps
    ));

    // Add a full circle using create_circle_arc
    win.add_object(create_circle_arc(
        Position::new(200.0, 100.0),    // center
        40.0,                           // radius
        Color::new(1.0, 0.5, 0.0, 1.0), // orange
        1.0,                            // z_index
        2.0,                            // line_width
        60,                             // steps
    ));

    win.add_object(create_heart_shape(
        Position::new(650.0, 300.0),
        60.0,                           // size
        Color::new(1.0, 0.0, 1.0, 1.0), // magenta
        1.0,                            // z_index
        2.0,                            // line_width
        100,                            // steps
    ));

    win.add_object(create_star_shape(
        StarShape {
            center: Position::new(150.0, 150.0),
            outer_radius: 50.0, // outer_radius
            inner_radius: 25.0, // inner_radius
            points: 5,
        }, // points
        Color::new(1.0, 1.0, 0.0, 1.0), // yellow
        1.0,                            // z_index
        2.0,                            // line_width
        20,                             // steps
    ));

    let path_points = vec![
        Position::new(400.0, 450.0),
        Position::new(450.0, 400.0),
        Position::new(500.0, 450.0),
        Position::new(550.0, 400.0),
        Position::new(600.0, 450.0),
    ];

    win.add_object(create_smooth_path(
        path_points,
        Color::new(0.5, 0.8, 0.2, 1.0), // lime green
        1.0,                            // z_index
        2.0,                            // line_width
        30,                             // steps
    ));

    win.add_object(
        PathBuilder::new()
            .move_to(Position::new(50.0, 300.0))
            .line_to(Position::new(100.0, 250.0))
            .quadratic_to(
                Position::new(150.0, 200.0), // control
                Position::new(200.0, 250.0), // end
            )
            .cubic_to(
                Position::new(250.0, 200.0), // control1
                Position::new(300.0, 300.0), // control2
                Position::new(350.0, 250.0), // end
            )
            .arc_to(
                Position::new(400.0, 200.0), // center
                50.0,                        // radius
                0.0,                         // start_angle
                std::f32::consts::PI,        // end_angle
            )
            .build(
                Color::new(0.8, 0.4, 0.8, 1.0), // purple
                1.0,                            // z_index
                2.5,                            // line_width
                40,                             // steps
            ),
    );

    win.launch();
}
