
fn loop_let_fun(){

    println!("by using the loop let function : ");

    // Make `optional` of type `Option<i32>`
    let mut optional = Some(0);

    // Repeatedly try this test.
    loop {
        match optional {
            // If `optional` destructures, evaluate the block.
            Some(i) => {
                if i > 3 {
                    println!("Greater than 3, quit!");
                    optional = None;
                } else {
                    println!("`i` is `{:?}`. Try again.", i);
                    optional = Some(i + 1);
                }
                // ^ Requires 3 indentations!
            },
            // Quit the loop when the destructure fails:
            _ => { break; }
            // ^ Why should this be required? There must be a better way!
        }
    }   
}

fn while_let_fun(){

    println!("by using the while let function ");
    let mut optional = Some(0);

    // This reads: "while `let` destructures `optional` into
    // `Some(i)`, evaluate the block (`{}`). Else `break`.
    while let Some(i) = optional {
        if i > 3 {
            println!("Greater than 3, quit!");
            optional = None;
        } else {
            println!("`i` is `{:?}`. Try again.", i);
            optional = Some(i + 1);
        }
        // ^ Less rightward drift and doesn't require
        // explicitly handling the failing case.
    }
}
pub fn loop_while_let_fun(){
    
    while_let_fun();
    loop_let_fun();
}