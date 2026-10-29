//string types
//two types
//string slice
//string

//String slicing = borrowing a part of a string using [start..end].

fn main() {
    //string
    let s = String::from("spiderman");
    //string slice
    let slice_string = &s[0..5];
    //take characters from index 0 to 4 (5 not included).
    println!("string : {s} \nstring slice : {slice_string}");
    
}
