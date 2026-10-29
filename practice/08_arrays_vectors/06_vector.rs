fn main() { let mut nums = vec![6, 8, 10]; nums.push(12); let sum: i32 = nums.iter().sum(); println!("{:?} sum={sum}", nums); }
