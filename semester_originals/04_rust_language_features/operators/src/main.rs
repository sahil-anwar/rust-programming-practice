#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_assignments)]

fn main() {
    //arthematic operations

    println!("print 17 mod 5 : {}", 17 % 5);

    //camparison operators , equality , inequality , relational
    let a = 10;
    let b = 20;

    print!(
        "a == b :{} , a != b : {} , a > b : {} , a < b : {} , a >= b : {} , a <= b : {}  ",
        a == b,
        a != b,
        a > b,
        a < b,
        a < !b,
        a > !b
    );
    //logical operators
    // ANd &&
    // OR  ||
    //not ~

    let x = 10;
    let y = 20;
    if (x > 5) && (x < 25) {
        println!("\ncondition satisfied ");
    } else {
        println!("\ncondition not satisfied");
    }

    //assignment operators
    //add and assign : +=
    //subtract and assign: -=
    //multiply and assign: *=
    //divide and assign: /=
    //raminader and assign : %=

    let mut p = 5;

    p += 5;
    p -= 5;
    p *= 5;
    p /= 5;
    p %= 5;

    //bitwise operators
    // and wise &
    //or wise  |
    //Xor ^
    //not ~
    //left shift <<
    //right shift >>

    let q: u8 = 4;
    println!("{}", x & x);
}
