use cfont::font::shape::TextTransform;
use cfont::font::style::{FontWeight, TextStyle};
use cfont::font::{render_simple_text, render_text};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let simple_geometry = render_simple_text(
        "Arial",
        "Hello, World!",
        24.0,
        [100.0, 100.0],
        [800.0, 600.0],
    )?;

    println!("Simple text vertices: {}", simple_geometry.vertices.len());
    println!("Simple text indices: {}", simple_geometry.indices.len());

    let bold_transform = TextTransform {
        font_size: 32.0,
        position: [50.0, 200.0],
        canvas_size: [800.0, 600.0],
        style: TextStyle::bold(),
    };

    let bold_geometry = render_text("Arial", "Bold Text!", bold_transform)?;
    println!("Bold text vertices: {}", bold_geometry.vertices.len());

    let italic_transform = TextTransform {
        font_size: 28.0,
        position: [50.0, 300.0],
        canvas_size: [800.0, 600.0],
        style: TextStyle::italic(),
    };

    let italic_geometry = render_text("Arial", "Italic Text!", italic_transform)?;
    println!("Italic text vertices: {}", italic_geometry.vertices.len());

    let underlined_transform = TextTransform {
        font_size: 24.0,
        position: [50.0, 400.0],
        canvas_size: [800.0, 600.0],
        style: TextStyle::underlined(),
    };

    let underlined_geometry = render_text("Arial", "Underlined Text!", underlined_transform)?;
    println!(
        "Underlined text vertices: {}",
        underlined_geometry.vertices.len()
    );

    let complex_style = TextStyle::new()
        .with_weight(FontWeight::ExtraBold)
        .with_italic(true)
        .with_underlined(true);

    let complex_transform = TextTransform {
        font_size: 30.0,
        position: [50.0, 500.0],
        canvas_size: [800.0, 600.0],
        style: complex_style,
    };

    let complex_geometry = render_text("Arial", "Complex Styled Text!", complex_transform)?;
    println!(
        "Complex styled text vertices: {}",
        complex_geometry.vertices.len()
    );

    // Show bounding boxes
    let (min_x, min_y, max_x, max_y) = complex_geometry.bounding_box();
    println!("Complex text bounding box: ({min_x}, {min_y}) to ({max_x}, {max_y})",);

    let (width, height) = complex_geometry.pixel_dimensions();
    println!("Complex text dimensions: {width}x{height} pixels");

    Ok(())
}
