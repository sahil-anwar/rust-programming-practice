use std::collections::HashMap;
fn main() { let mut marks = HashMap::new(); marks.insert("quiz", 15); marks.insert("lab", 12); println!("{:?}", marks.get("quiz")); }
