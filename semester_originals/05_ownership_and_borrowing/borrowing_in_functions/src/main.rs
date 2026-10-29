fn borrow_vec(vec: &Vec<i32>) {
    println!("vector 1 is : {:?}", vec)
}

fn mutably_borrow_vec(vec: &mut Vec<i32>) -> &Vec<i32> {
    vec.push(10);
    vec
}

fn main() {
    //funtions that immutably borrows values
    let vec1: Vec<i32> = vec![1, 2, 3, 4, 5];
    let ref1 = &vec1;
    borrow_vec(ref1);
    println!("vec 1 is : {:?}", ref1);

    //funtions that mutably borrows values
    let mut vec2: Vec<i32> = vec![2, 3, 4, 6, 7];
    let ref2 = &mut vec2;
    mutably_borrow_vec(ref2);
    println!("ref 2 : {:?}", ref2)
}
