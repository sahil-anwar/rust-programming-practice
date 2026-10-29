fn main() {
    let x = 10.0;
    let x = 0.9;

    println!("x:{x}");

    let mut y = 5;
    y = 7;
    println!("{y}");

    {
        let z = 5;
    }
    //let s = z;

    //shadowing
    let t = 10;
    let t = t + 10;

    //constant
    const MAX_VALUE: u32 = 100;

    //type alisaing
    type Age = u32;
    let x: Age = 20;

    //type conversion
    let q: u32 = 10;
    let p = q as f32;


}
