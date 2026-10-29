use std::collections::HashMap;
fn main() { let mut marks = HashMap::new(); marks.insert("quiz", 20); marks.insert("lab", 16); println!("{:?}", marks.get("quiz")); }
