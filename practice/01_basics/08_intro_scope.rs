fn main() { let x = 3; { let x = x + 4; println!("inner {x}"); } println!("outer {x}"); }
