fn length(text: &str) -> usize { text.len() }
fn main() { let name = String::from("ownership2"); let size = length(&name); println!("{name} has {size} bytes"); }
