fn main() { let mut nums = vec![1, 3, 5]; nums.push(7); let sum: i32 = nums.iter().sum(); println!("{:?} sum={sum}", nums); }
