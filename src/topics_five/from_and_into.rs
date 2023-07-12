use std::convert::From;
#[allow(dead_code)]
#[derive(Debug)]
struct Number{
    value:i32
}

impl From<i32> for Number{
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

pub fn from_and_into_fun()
{
    let num = Number::from(30);//explicit conversion
    println!("number is : {:?} ",num);

    //now for into trait 
    let my_int = 5;
    let my_num:Number = my_int.into(); //implicit conversion integer to struct

    println!("using into trait my_num {:?} ",my_num);

    /*
        Into used when conversion source type to target is implicit 
        From being used when conversion source type to target is explicit 
    
    */ 


}