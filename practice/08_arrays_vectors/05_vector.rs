fn main() { let mut nums = vec![5, 7, 9]; nums.push(11); let sum: i32 = nums.iter().sum(); println!("{:?} sum={sum}", nums); }
