
   type MyAge = u32;
   type RollNumber = u32;
   type HeigthCm = u32;
   type WeigthCm = u32 ;
   
fn print_person(age : MyAge ,roll_no : RollNumber,height : HeigthCm, weigth : WeigthCm )
{
    println!(" Age : {} , Roll Number : {} , Height : {} cm ,weigth : {} kg ",age,roll_no,height,weigth);
}

fn main()
{
    print_person(23,9141,31,68);
}