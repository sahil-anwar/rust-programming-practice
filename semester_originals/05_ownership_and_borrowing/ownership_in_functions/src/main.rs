//ownership in funtions
#![allow(unused_assignments)]
fn takes_ownership(vec: Vec<i32>) {
    println!("vec is : {:?}", vec);
}
//gives ownership
fn gives_ownership() -> Vec<i32> {
    vec![2, 3, 4, 5]
}
//take and return ownership
fn take_and_return_ownership(mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(10);
    vec
}
//stack data
fn stack_data(mut var: i32) {
    var = 56;
    println!("in funtion the var is : {}", var);
}
fn main() {
    // 1- funtions that takes ownership
    let vec_1: Vec<i32> = vec![1, 2, 3, 5, 6];
    //takes_ownership(vec_1);
    takes_ownership(vec_1.clone());
    println!("vector after funtion call: {:?}", vec_1);

    // 2- funtions that gives ownership

    let vec_2 = gives_ownership();
    println!("vec2 : {:?}", vec_2);

    let vec_3 = vec![2, 3, 4, 5, 6];
    println!("vector 3 : {:?}", take_and_return_ownership(vec_3.clone()));
    println!("vector 3 after calling funtion : {:?}", vec_3);

    //funtions that take stack data

    let x = 10;
    stack_data(x);
    println!("the var in main is : {x}");

    let mut my_vec = vec![1, 2, 3, 4, 5];
    let mut temp;

    while !my_vec.is_empty() {
        temp = my_vec; // Something wrong on this line
        println!("Elements in temporary vector are: {:?}", temp);
        println!("Popped element: {}", my_vec.pop().unwrap()); // pop() is used to remove an element from the vec. The unwrap() is used to return the value inside the Some() variant
    }
}
