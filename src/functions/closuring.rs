use std::mem;

fn capturing_fun(){
    let color = String::from("green");
    let print = || println!("`color`: {:?} ",color);
    print();
    
    let mut count = 0;
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    inc();
    inc();

    // let _count_reborrowed = &mut count;  

    // A non-copy type.
    let movable = Box::new(3);

    // `mem::drop` requires `T` so this must take by value. A copy type
    // would copy into the closure leaving the original untouched.
    // A non-copy must move and so `movable` immediately moves into
    // the closure.
    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };

    // `consume` consumes the variable so this can only be called once.
    consume();
    // consume(); because movable is  destroy  



}

fn capturing2_fun(){
    let haystack = vec![1,2,3];
    let contains = move |needle| haystack.contains(needle);

    println!("{}",contains(&1));
    println!("{}",contains(&4));

}


pub fn closuring_fun(){

    let outer_var = 5;

    let closure_annotated = |i:i32| ->i32 {i+outer_var};
    let closure_inferred = |i|  i+outer_var;

    println!("Closure annotated : {:?} ",closure_annotated(2));
    println!("Closure inferred : {:?} ",closure_inferred(1));

    let one = || 1;
    println!("closure return one : {} ",one());

    /*
    Closure is function that can capture the outside the enclosing environment
    above by using function we cant access to outvar in a func 
    */

    let closure_arm = "capturing2";

    match closure_arm{
        "capturing" => { capturing_fun();},
        "capturing2" => { capturing2_fun();},
        _ =>{},
    }


}