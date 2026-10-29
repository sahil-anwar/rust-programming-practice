fn main() { let input = "13"; let parsed: Result<i32, _> = input.parse(); match parsed { Ok(n) => println!("number={n}"), Err(e) => println!("error={e}") } }
