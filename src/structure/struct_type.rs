//1.unit type struct
struct UnitStruct;

//2. tuple type struct
#[derive(Debug)]
struct Tuple(i32,f32,char,String);

//3. classics C struct

struct MyStruct{
    name:String,
    id:i32,
    tup:Tuple,
}


trait DisplayStruct{
  fn display_fun(&self);
}

impl DisplayStruct for MyStruct{
    fn display_fun(&self){
        println!("Name : {}",self.name);
        println!("Id: {}",self.id);
        println!("tuple : {:?} ",self.tup);
    }
    
}

impl DisplayStruct for UnitStruct{
    fn display_fun(&self) {
        println!("unit struct function example...");
    }
}

pub fn struct_fun(){

    let tuple_1 = Tuple(12,1.5,'A',String::from("string"));

    let struct_obj = MyStruct{
      name: String::from("Structures"),
      id: 123,
      tup: tuple_1,
    };
    let unit_struct = UnitStruct;

    struct_obj.display_fun();
    
    unit_struct.display_fun();

}

