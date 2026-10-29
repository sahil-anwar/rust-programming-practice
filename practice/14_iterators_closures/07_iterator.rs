fn main() { let nums = [1, 2, 3, 4, 5]; let result: Vec<i32> = nums.iter().map(|n| n * 7).filter(|n| n % 2 == 0).collect(); println!("{:?}", result); }
