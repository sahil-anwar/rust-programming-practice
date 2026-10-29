fn largest<T: Ord + Copy>(a: T, b: T) -> T { if a > b { a } else { b } }
fn main() { println!("{}", largest(12, 16)); }
