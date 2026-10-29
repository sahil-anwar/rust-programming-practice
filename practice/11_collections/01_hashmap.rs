use std::collections::HashMap;
fn main() { let mut marks = HashMap::new(); marks.insert("quiz", 5); marks.insert("lab", 4); println!("{:?}", marks.get("quiz")); }
