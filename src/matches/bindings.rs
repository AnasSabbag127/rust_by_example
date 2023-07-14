fn age() -> u32{
    19
}


fn fun1(){
    println!(" age in class :  ");
    match age(){
        0 => { println!(" infant ");},
        n @ 1..= 12 => { println!("child : {}",n);},
        n @ 13..=19 => {println!("teens :{}",n);},
        n => { println!("above adult "); },
    }
}

fn some_number() -> Option<u32>{
    Some(34)
}

fn fun2(){
    match some_number(){
        Some(n @34) => { println!(" correct number : {} ",n);},
        Some(n) => {println!(" not correct trye again :  {n} ");}
        _ => { println!(" none default ");},
    }
}
pub fn bindings_fun(){
    
    fun1();
    fun2();

}