fn print_num(mut x: i32) {
    x = x + 2;
    println!("{x}");
}
fn print_num2(x: i32) {
    let mut x = x;
    println!("{x}");
}

fn main() {
    let mut num = 10;
    print_num(num);
    print_num2(num);
}
