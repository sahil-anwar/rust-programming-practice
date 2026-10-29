fn main() { let mut nums = vec![4, 6, 8]; nums.push(10); let sum: i32 = nums.iter().sum(); println!("{:?} sum={sum}", nums); }
