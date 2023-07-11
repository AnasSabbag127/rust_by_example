fn fun_1(){
    // This binding lives in the main function
    let long_lived_binding = 1;

    // This is a block, and has a smaller scope than the main function
    {
        // This binding only exists in this block
      let  short_lived_binding = 2;

        println!("inner short: {}", short_lived_binding);
    }
    // End of the block

    // Error! `short_lived_binding` doesn't exist in this scope
    // println!("outer short: {}", short_lived_binding);
    // FIXME ^ Comment out this line

    println!("outer long: {}", long_lived_binding);
}

fn fun_2(){
    let shadowed_binding = 1;

    {
        println!("before being shadowed: {}", shadowed_binding);

        // This binding *shadows* the outer one
        let shadowed_binding = "abc";


        println!("shadowed in inner block: {}", shadowed_binding);
    }
    println!("outside inner block: {}", shadowed_binding);

    // This binding *shadows* the previous binding
    let shadowed_binding = 2;
    println!("shadowed in outer block: {}", shadowed_binding);
}

fn fun_3()
{
    let mut _mutable_integer = 7i32;
    {
        // Shadowing by immutable `_mutable_integer`
        let _mutable_integer = _mutable_integer;
        println!("_mutable_ integer inner scope : {} ",_mutable_integer);
        // Error! `_mutable_integer` is frozen in this scope
        // _mutable_integer = 50;
        // FIXME ^ Comment out this line

        // `_mutable_integer` goes out of scope
    }
    println!("_mutable_ integer outer scope : {} ",_mutable_integer);
    // Ok! `_mutable_integer` is not frozen in this scope
    _mutable_integer = 3;

    // When data is bound by the same name immutably, it also freezes.
    // Frozen data can't be modified until the immutable binding goes out of scope:
}
pub fn scope_and_shadow_fun(){

    fun_1();
    fun_2();
    //Declairing first in after initialising not allowed in rust
    fun_3();
}