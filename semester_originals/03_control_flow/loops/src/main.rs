fn main() {
    'outer: loop {
        loop {
            println!("i am sahil");
            break 'outer;
        }
    }
    let vec: Vec<i32> = vec![32, 43, 23, 5, 3, 24, 5];
    for i in vec {
        println!("{i}");
    }

    //while loop
    let mut n = 11;
    while n < 10 {
        n = n + 1;
        println!("1");
    }

    //for loop (asd decending)
    //infinite loop for i in 0..=5

    // for i in (0..=5).rev() {
    //     println!("i : {i}");
    // }

    //iterative step loop

    for i in (0..=10).step_by(5) {
        println!("i: {i}");
    }

    for i in (0..100).step_by(10) {
        println!("i: {i}");
    }
}
