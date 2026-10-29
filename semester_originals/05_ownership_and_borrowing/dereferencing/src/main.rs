fn main() {
    //deferencing stack allocated types
    let mut var = 56;
    let ref1 = &mut var;
    let deref = *ref1;
    *ref1 = 12;
    print!("varibale : {var} , deref : {deref}");

    //deferencing stack allocated
    let mut heap_data = vec![12, 3, 4, 5];
    let ref_heap_data = &mut heap_data;
    //let deref_copy = *ref_heap_data;
    ref_heap_data.push(8);
    (*ref_heap_data).push(8);
}
