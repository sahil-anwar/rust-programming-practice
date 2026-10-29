use std::fs;
fn main() -> std::io::Result<()> { let path = std::env::temp_dir().join("rust_practice_2.txt"); fs::write(&path, "practice 2")?; println!("{}", fs::read_to_string(&path)?); fs::remove_file(path)?; Ok(()) }
