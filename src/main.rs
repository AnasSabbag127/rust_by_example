mod topics;
mod topics_two;
use topics::{hello_word,debug_format,display_format};
use topics_two::{array_topic, tuple_topic, literal_topic};
mod structure;
mod enums;
use structure::struct_type;
use crate::enums::enum_example;//crate represent from root path to given path
mod topics_four;
use topics_four::{mutability_var,scope_and_shadow};

use std::env;



fn main() {

let args: Vec<String> = env::args().collect();
let list_topic1 = r#"1. debug_format_fn
2. display_format_fn
3. hello_world_fn"#;





let list_topic2:&str = r#"1.array
2.literal and operator
3.tuple "#;
let topic_list3 = r#"1.structure
2.Enum
3.constant"#;

let topic_list4=r#"1.mutablitiy
2.scope shadowing
3.Declair first
4.Freezing "#;

let current_topic = "variable_binding";

match current_topic{
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
    "custom_type" => { 
            match args[1].as_str(){
                "struct_fun" => { struct_type::struct_fun(); },
                "enum_fun" => { enum_example::enum_fun(); }
                "list" => { println!("topic_list: {:?}",topic_list3); },
                _  => { println!("Invalid arguement .");}
            }
        
        },
    "variable_binding" => { 
        match args[1].as_str(){
            "mutability_fun" => { mutability_var::mutability_fun();},
            "scope_and_shadowing_fun" => {scope_and_shadow::scope_and_shadow_fun(); }
            "list" => { println!("topic_list: {:?}",topic_list4); },
            _  => { println!("Invalid arguement .");}
        }
    
    },

    _ => {println!("invalid topic name ");}
    
    }


}






