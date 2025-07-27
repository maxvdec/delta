use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

fn main() {
    let shader_dir = Path::new("metal");
    let out_file = Path::new("src/macos/shader_code.rs");

    println!("cargo:rerun-if-changed={}", shader_dir.display());

    let mut shader_code = String::new();

    collect_shaders(shader_dir, &mut shader_code);

    let shader_code_literal = shader_code.escape_default().to_string();

    let mut file = File::create(out_file).expect("Failed to create shader_code.rs");
    writeln!(
        file,
        "// Auto-generated from 'metal/' directory\n\
         pub const SHADER_CODE: &str = \"{shader_code_literal}\";"
    )
    .expect("Failed to write shader code to file");
}

fn collect_shaders(dir: &Path, output: &mut String) {
    for entry in fs::read_dir(dir).expect("Failed to read shader directory") {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();

        if path.is_dir() {
            collect_shaders(&path, output);
        } else if let Some(ext) = path.extension() {
            if ext == "metal" {
                let content = fs::read_to_string(&path)
                    .unwrap_or_else(|_| panic!("Failed to read file {path:?}"));
                output.push_str(&format!("// File: {}\n{}\n\n", path.display(), content));
            }
        }
    }
}
