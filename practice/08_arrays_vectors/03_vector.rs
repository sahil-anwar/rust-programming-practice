fn main() { let mut nums = vec![3, 5, 7]; nums.push(9); let sum: i32 = nums.iter().sum(); println!("{:?} sum={sum}", nums); }
