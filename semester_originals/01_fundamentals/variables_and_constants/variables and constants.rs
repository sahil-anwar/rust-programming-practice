fn main() {
    // definition of variable
    // binding
    // here x is binding which holds the value 10.
    let x: i32 = 10;
    println!("x is {x}");
    //-------------------------------------------
    //              Mutability
    //-------------------------------------------
    let mut y = 5;
    println!("y is {y}");

    y = 7;
    println!("y is {y}");

    //-------------------------------------------
    //              Scopes
    //-------------------------------------------
    {
        let s = 9141;
        println!("s is {s}");
    }
    //let p = s;
    //println!("p is {p}");
    // line 23 will show error bcs it's out of scope

    //-------------------------------------------
    //              Shadowing
    //-------------------------------------------

    let t = 10;
    println!("first 't' is : {t}");
    let t = t + 10;
    println!("second 't' is : {t}");

    // let u = 3;
    // let u = 3.0;

    //nested scope
    let v = 40;
    {
        let v = 30;
        println!(" inner v is: {v}");
    }
    println!("outter v is:{v} ");

    /*
    -------------------------------------------
                 Constants
    -------------------------------------------
    */
    const MAX_VALU: u32 = 10;
    println!("the const is: {MAX_VALU}");
}
