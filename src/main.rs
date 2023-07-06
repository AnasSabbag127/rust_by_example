mod topics;
use topics::{hello_word,debug_format,display_format};
fn main() {
    // println!("Hello, world!");
    // hello_word::formatted_print();

    // println!("{0},this is one {1} this is two {2}","zero","One","Two");
    // println!("{obj1} {str} {str2} ",obj1 = "hii",str = "123",str2 = "abc");
    // println!("{number:0<5}", number=111111);
    // println!("{}{1}",12,"dbdb");
    // debug_format::debug_format_fun2();

    display_format::display_format_fun();

}   
