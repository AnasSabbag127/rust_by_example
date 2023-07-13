fn while_loop_fun(){
    let mut n = 1;
    // Loop while `n` is less than 16
    while n < 16 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }

        // Increment counter
        n += 1;
    }
}

fn for_range_fun(){
    
    // `n` will take the values: 1, 2, ..., 16 in each iteration
    for n in 1..16 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
}
fn for_range_fun2(){
    // `n` will take the values: 1, 2, ..., 10 in each iteration
    for n in 1..=15 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
}

fn for_iter(){
    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            // TODO ^ Try deleting the & and matching just "Ferris"
            _ => println!("Hello {}", name),
        }
    }
    
    println!("names: {:?}", names);
}

fn for_into_iter(){
    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    
    // println!("names: {:?}", names);
    // FIXME ^ Comment out this line
}

fn for_iter_mut(){
    let mut names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }

    println!("names: {:?}", names);

}


pub fn for_and_while_loops_fun()
{
    println!(" for and while loop function ");
    let fun_name = "while";
    match fun_name {
        "while" => { while_loop_fun(); },
        "for_range" => { for_range_fun(); },
        "for_range2" => { for_range_fun2(); },
        //iterator for loop
        "for_iter" => { for_iter(); },
        "for_into_iter" => { for_into_iter(); },
        "for_iter_mut" => { for_iter_mut();},
        _ => {}
    }
        
}