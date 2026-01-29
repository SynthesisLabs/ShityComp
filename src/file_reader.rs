use std::fs::File;
use std::io::Read;
pub fn read_file_to_string(file_path: &str) -> std::io::Result<String> {
    println!("Reading file: {}", file_path);
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}