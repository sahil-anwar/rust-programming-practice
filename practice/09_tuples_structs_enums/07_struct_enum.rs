#[derive(Debug)]
struct Student { id: u32, grade: u8 }
fn main() { let s = Student { id: 7, grade: 77 }; println!("{:?}", s); }
