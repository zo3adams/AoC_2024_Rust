use std::{fs, path::Path};

fn main() {
    let out_file = std::path::Path::new("src/aoc_index.rs");
    let mut out = String::new();

    out.push_str("pub fn aoc_registry() -> std::collections::HashMap<String, fn()> {\n");
    out.push_str("    aoc_registry!(\n");

    for year in ["y2024", "y2025"] {
        let dir = Path::new("src").join(year);
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries {
                let entry = entry.unwrap();
                let name = entry.file_name().to_string_lossy().into_owned();
                if name.ends_with(".rs") && name.starts_with("day_") {
                    let day: u32 = name[4..6].parse().unwrap();
                    let year_num: u32 = year[1..].parse().unwrap();
                    let path = format!("\"{}/{name}\"", year);
                    out.push_str(&format!("        ({year_num}, {day}, {path}),\n"));
                }
            }
        }
    }

    out.push_str("    )\n}\n");

    fs::write(out_file, out).unwrap();
    println!("cargo:rerun-if-changed=src");

}
