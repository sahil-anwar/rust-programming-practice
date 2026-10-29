use std::collections::HashMap;
fn main() { let mut marks = HashMap::new(); marks.insert("quiz", 25); marks.insert("lab", 20); println!("{:?}", marks.get("quiz")); }
