fn main() { let mut nums = vec![7, 9, 11]; nums.push(13); let sum: i32 = nums.iter().sum(); println!("{:?} sum={sum}", nums); }
