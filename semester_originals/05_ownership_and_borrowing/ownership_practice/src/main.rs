#![allow(dead_code)]
#![allow(unused_variables)]
fn main() {
    //primitive -> copied , no error issue
    //intergers
    //floats
    //boolean
    //character
    //primitive compound data types
    //-> tuples , arrays

    //integer
    let s1: i32 = 30;
    let s2: i32 = s1; //copied
    println!("{s1}");

    //float
    let f1 = 0.5;
    let f2 = f1; //copied
    println!("f2: {f2}");

    // boolean
    let is_passed: bool = true;
    let result: bool = is_passed; // copied
    println!("{is_passed}");

    //char
    let grade: char = 'A';
    let final_grade: char = grade; // copied
    println!("{grade}");

    //array
    let arr1: [i32; 3] = [10, 20, 30];
    let arr2 = arr1; // copied
    println!("{:?}", arr1);

    //tuple
    let tup1: (i32, bool, char) = (10, true, 'A');
    let tup2 = tup1; // copied
    println!("{:?}", tup1);

    //Non - Primitive
    //string-----------
    let s1 = String::from("hello");
    let s2 = s1; // moved
                 // println!("{s1}"); // ❌ error

    //vectors-------------
    let v1 = vec![1, 2, 3];
    let v2 = v1; // moved
                 // println!("{:?}", v1); // ❌ error

                 
}
