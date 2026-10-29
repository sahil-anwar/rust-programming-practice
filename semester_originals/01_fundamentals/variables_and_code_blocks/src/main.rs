fn my_function(name: &str, age: u8, roll_number: i32) {
    println!(
        "name : {} , age : {} , roll number: {}",
        name, age, roll_number
    );
}
fn my_fn2(name: &str) {
    println!("fn2: {}", name);
}

//multiplication
fn multiplication(num1: i32, num2: i32) -> i32 {
    //return 45;
    println!("computiong multiplication");
    num1 * num2
}
//code blocks


fn main() {
    my_function("sahil", 23, 9141);
    let name = " sahil anwar";
    my_fn2(name);
    //multiplication(10, 90);
    let answer = multiplication(30, 40);
    println!("answer : {}", answer);

    let full_name: String = {
        let first_name = "sahil";
        let last_name= "anwar";
        format!("{first_name} {last_name}")
    }
    println!("{full_name}");
}
