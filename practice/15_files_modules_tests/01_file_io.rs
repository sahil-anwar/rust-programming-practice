use std::fs;
fn main() -> std::io::Result<()> { let path = std::env::temp_dir().join("rust_practice_1.txt"); fs::write(&path, "practice 1")?; println!("{}", fs::read_to_string(&path)?); fs::remove_file(path)?; Ok(()) }
