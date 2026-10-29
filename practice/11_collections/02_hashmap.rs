use std::collections::HashMap;
fn main() { let mut marks = HashMap::new(); marks.insert("quiz", 10); marks.insert("lab", 8); println!("{:?}", marks.get("quiz")); }
