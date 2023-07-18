

fn fun1(){
    let number = Some(23);
    let letter:Option<i32> = None;
    match number {
        Some(i) => {
            println!(" we have the number : {:?} ",i);
        },
        _ => { println!(" we dont have the number : {:?} ",number );}//here if satisfy it will return None
    }

    match letter {
        Some(i) => {
            println!(" we have the number  : {:?} ",i);
        },
        _ => { println!(" we  have the letter : {:?} ",letter );}//here if satisfy it will return None
    }

}

fn fun2(){
    println!(" if let example 2 : ");
    
    #[derive(Debug)]
    enum Foo {
        Bar,
        Baz,
        Qux(u32)
    }

    // Create example variables
    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);
    
    // Variable a matches Foo::Bar
    if let Foo::Bar = a {
        println!("a is foobar");
    }

    if let Foo::Baz = b {
        println!("b is foobaz");
    }

    // Variable b does not match Foo::Bar
    // So this will print nothing
    if let Foo::Bar = b {
        println!("b is foobar");
    }

    
    // Variable c matches Foo::Qux which has a value
    // Similar to Some() in the previous example
    if let Foo::Qux(value) = c {
        println!("c is {}", value);
    }

    // Binding also works with `if let`
    if let Foo::Qux(value @ 100) = c {
        println!("c is one hundred {:?} ",value);
    }




}
pub fn if_let_fun(){

    fun1();
    fun2();

}