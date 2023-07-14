pub fn guards_fun(){

    enum Temp{
        Celc(i32),
        Fahren(i32),
    }

    let temp = Temp::Celc(40);

    match temp{
        Temp::Celc(t) if t>35 => { println!(" too hot in cels");},
        Temp::Celc(t)  => { println!("NOT too hot in cels");},//its else condition
    
        Temp::Fahren(t) if t>70 => {println!("too hot from fahrenhite ");},
        Temp::Fahren(t) => {println!("not too hot in fahren ");},
        _  => { println!(" default ");} 
    }

}