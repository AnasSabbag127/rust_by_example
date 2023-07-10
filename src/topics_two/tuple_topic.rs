pub fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // `let` can be used to bind the members of a tuple to variables.
    let (int_param, bool_param) = pair;
    (bool_param, int_param)
}


pub fn tuple_fun()
{
    let tup =(1,2,'a','b');
    println!("tuple : {:?} ",tup);
    let tup_new = (23,true);
    println!("before reverse the tuple is : {:?} ",tup_new);
    let tup_new_by_reverse = reverse(tup_new);//change the order of tuple of pair type
    println!("after reversing the tuple is : {:?} ",tup_new_by_reverse);
    
    let tuple = (1, "hello", 4.5, true);
    let (a, b, c, d) = tuple;

    println!("{:?}, {:?}, {:?} , {:?} ", a, b, c, d);
    
    // slice operation are not support by tuple because of discontinuous memory for items
    // let tuple_slice = &tuple[..2];



}
