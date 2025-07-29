use cfont::font::{
    load::get_system_font,
    shape::{TextTransform, produce_text},
    style::TextStyle,
};

#[test]
fn render_text() {
    let font = get_system_font("Arial").unwrap();
    let mut result = produce_text(font.clone(), "Hello, World!").unwrap();

    println!("=== BEFORE TRANSFORMATION ===");
    let (min_x, min_y, max_x, max_y) = result.bounding_box();
    println!("Raw font units bounding box: ({min_x:.1}, {min_y:.1}) to ({max_x:.1}, {max_y:.1})",);
    println!(
        "Raw dimensions: {:.1} x {:.1} font units",
        max_x - min_x,
        max_y - min_y
    );
    println!("Total vertices: {}", result.vertices.len());

    println!("First 3 vertices (font units):");
    for (i, vertex) in result.vertices.iter().take(3).enumerate() {
        println!(
            "  [{}]: ({:.1}, {:.1})",
            i, vertex.position[0], vertex.position[1]
        );
    }

    let font_units_per_em = 1000.0;

    let transform = TextTransform {
        font_size: 48.0,             // 48px font
        position: [100.0, 200.0],    // Position at 100px from left, 200px from top
        canvas_size: [800.0, 600.0], // 800x600 canvas
        style: TextStyle::bold(),
    };

    result.transform_to_canvas(transform.clone(), font_units_per_em);

    println!("\n=== AFTER TRANSFORMATION ===");
    let (min_x, min_y, max_x, max_y) = result.bounding_box();
    println!(
        "Pixel coordinates bounding box: ({min_x:.1}, {min_y:.1}) to ({max_x:.1}, {max_y:.1})",
    );
    println!(
        "Pixel dimensions: {:.1} x {:.1} pixels",
        max_x - min_x,
        max_y - min_y
    );

    println!("First 3 vertices (pixels):");
    for (i, vertex) in result.vertices.iter().take(3).enumerate() {
        println!(
            "  [{}]: ({:.1}, {:.1})",
            i, vertex.position[0], vertex.position[1]
        );
    }

    println!("\n=== POSITIONING VERIFICATION ===");
    println!(
        "Text should start around position: ({:.1}, {:.1})",
        transform.position[0], transform.position[1]
    );
    println!("Actual text starts at: ({min_x:.1}, {min_y:.1})");
}
