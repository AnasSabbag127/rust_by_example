#[derive(Debug)]
struct Structure(f32);

// Put a `Structure` inside of the structure `Deep`. Make it printable
// also.
#[derive(Debug)]
struct Deep(Structure);

// #[derive(Debug)]
// struct Person<'a> {
//     name: &'a str,
//     age: i32
// }


pub fn debug_format_fun()
{
    // Printing with `{:?}` is similar to with `{}`.
    println!("{} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name.",
             "Slater",
             "Christian",
             actor="actor's");

    // `Structure` is printable!
    println!("Now {:?} will print!", Structure(3.21));

    // The problem with `derive` is there is no control over how
    // the results look. What if I want this to just show a `7`?
    println!("Now {:?} will print!", Deep(Structure(7.3)));


    /*because it requires impl display macro prelude */
    // let name = "Peter";
    // let age = 27;
    // let peter = Person { name, age };
    // // Pretty print
    // println!(" for pretty print : {:#?}", peter);


}

