fn main() { let mut nums = vec![2, 4, 6]; nums.push(8); let sum: i32 = nums.iter().sum(); println!("{:?} sum={sum}", nums); }
