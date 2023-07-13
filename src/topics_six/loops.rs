#[allow(unused_labels)]
#[allow(unreachable_code)]
fn single_loop()
{
    let mut count = 0u32;

    println!("Let's count until infinity!");

    // Infinite loop
    loop {
        count += 1;

        if count == 3 {
            println!("three");

            // Skip the rest of this iteration
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");

            // Exit this loop
            break;
        }
    }
}


pub fn nesting_and_label_loops(){
    // let x=1;
    // let y=1;
    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");

            // This would break only the inner loop
            // break;

            // This breaks the outer loop
            break 'outer;
        }
        // if x==5 {
          println!("This point will never be reached");   
        // }   
        // x+=1;
    }

    println!("Exited the outer loop");

}

pub fn returning_from_loop(){
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("result: {result} ");


}
pub fn loop_funs(){
    single_loop();
    nesting_and_label_loops();
    returning_from_loop();

}