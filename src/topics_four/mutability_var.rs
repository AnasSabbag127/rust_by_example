
fn fun_mut(){
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);

    // Ok
    mutable_binding += 1;

    println!("After mutation: {}", mutable_binding);

    // Error! Cannot assign a new value to an immutable variable
    // _immutable_binding += 1;
}

pub fn mutability_fun(){

    let an_integer = 1u32;
    let a_boolean = true;
    
    let unit = ();

    println!("an_integer : {} ",an_integer);
    // // copy `an_integer` into `copied_integer`
    let copied_integer = an_integer;
    
    //** note an_integer is still valid because size of the integer 
    // ** is known at compiler time    
    println!("An copy integer: {:?} {}", copied_integer,an_integer);
    println!("A boolean: {}", a_boolean);
    println!("Meet the unit value: {:?}", unit);

    // The compiler warns about unused variable bindings; these warnings can
    // be silenced by prefixing the variable name with an underscore
    let _unused_variable = 3u32;

    // let noisy_unused_variable = 2u32;

    // FIXME ^ Prefix with an underscore to suppress the warning
    // Please note that warnings may not be shown in a browser

    fun_mut();
}