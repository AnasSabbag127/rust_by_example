
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
mod topics_five;
use topics_five::{type_cast,from_and_into,to_and_from_strings,try_from_and_try_into};
mod topics_six;
use topics_six::{expression,flow_control,loops,for_and_while_loops};
mod matches;
use matches::{bindings,guards,destructing};


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

let topic_list5=r#"1.type casting
2.from and into 
3.TryFrom and TryInto
4.To and from Strings "#;

let topic_list6 =r#"1.Expression
2.Flow Control: if/else ,match, if let ,else let ,while let 
3.Loops::{ while ,for , loop}"#;

let current_topic = "matches";

let topic_list7 =r#"1.destructing {tuple,array,enum,struct}
2.Guards
3.bindings "#;


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
    "conversion" =>{
        match args[1].as_str(){
            "type_cast_fun" => { type_cast::type_cast_fun();},
            "from_and_into_fun" => { from_and_into::from_and_into_fun(); },
            "TryFrom_and_TryInto_fun" => { try_from_and_try_into::try_from_and_try_into_fun();}
            "to_and_from_strings_fun" =>{ to_and_from_strings::to_and_from_strings_fun();}
            "list" => { println!("topic_list5: {:?}",topic_list5); },
            _  => { println!("Invalid arguement .");}
        }
    },
    "flow_control" =>{
        match args[1].as_str(){
            "expression_fun" => {expression::expression_fun();},
            "flow_control" => {flow_control::flow_control_funs();},
            "loops" => {loops::loop_funs();},
            "for_and_while_loop" => {for_and_while_loops::for_and_while_loops_fun();},
            "list" => { println!("topic_list6: {:?}",topic_list6); },
            _  => { println!("Invalid arguement .");}
        }
    },
    "matches" =>{
        match args[1].as_str(){//.....work here...
            "destructing" => {destructing::destructing_fun();},
            "guards" => {guards::guards_fun();},
            "bindings" => {bindings::bindings_fun();},
            "list" => { println!("topic_list7: {:?}",topic_list7); },
            _  => { println!("Invalid arguement .");}
        }
    },
    _ => { println!("invalid topic name "); }
    
    }


}






