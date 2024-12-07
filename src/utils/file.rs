use std::fs;
use std::path::Path;

pub fn read_input(day: u8, part: u8) -> String {
    let file_path = Path::new("src")
        .join("input")
        .join(format!("day{day}"))
        .join(format!("part{part}.txt"));

    fs::read_to_string(&file_path)
        .unwrap_or_else(|err| panic!("Failed to read input file at {:?}: {}", file_path, err))
}
