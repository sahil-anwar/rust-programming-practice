use std::collections::HashMap;
fn main() { let mut marks = HashMap::new(); marks.insert("quiz", 35); marks.insert("lab", 28); println!("{:?}", marks.get("quiz")); }
