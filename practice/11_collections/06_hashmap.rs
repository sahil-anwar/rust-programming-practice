use std::collections::HashMap;
fn main() { let mut marks = HashMap::new(); marks.insert("quiz", 30); marks.insert("lab", 24); println!("{:?}", marks.get("quiz")); }
