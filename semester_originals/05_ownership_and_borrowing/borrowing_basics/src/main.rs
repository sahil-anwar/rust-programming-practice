fn main() {
    let mut vec1 = vec![3, 3, 5];
    let ref1 = &vec1;
    let ref2 = &vec1;

    print!("the ref1 is : {:?}, reference 2 is {:?}", ref1, ref2);

    let ref3: &mut Vec<i32> = &mut vec1;

    // let vec2:  = {
    //     let vec3 = vec![12,4,5];
    //     &vec3
    // }
}
