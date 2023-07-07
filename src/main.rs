mod topics;
mod topics_two;
use topics::{hello_word,debug_format,display_format};
use topics_two::{array_topic, tuple_topic, literal_topic};
use std::env;
fn main() {

let args: Vec<String> = env::args().collect();
let list_topic1 = r#"1. debug_format_fn
2. display_format_fn
3. hello_world_fn"#;



let topic = "primitives";
let list_topic2:&str = r#"1.array
2.literal and operator
3.tuple "#;

match topic{
   "hello_world" => {
            match args[1].as_str() {
                "debug_format_fn" =>   { debug_format::debug_format_fun(); },
                "display_format_fn" => { display_format::display_format_fun(); },
                "hello_world_fn" => { hello_word::hello_world_fun(); },
                "list" => { println!("{list_topic1}"); },
                _ => {println!("enter function name ")},//default 
            }
        },
    "primitives" => {
            // in match key arm we run the function with command line arg key
            match args[1].as_str() {
                "array_fn" => { array_topic::array_fun(); },
                "tuple_fn" => { tuple_topic::tuple_fun();},
                "literal_fn" => { literal_topic::literal_fun();}, 
                "list" => { println!("{list_topic2}"); },
                _ => {println!("enter function name ")},//default 
            }
        },
        _ => {println!("invalid topic name ");}
    }


}






