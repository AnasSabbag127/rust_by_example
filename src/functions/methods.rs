struct Point{
    x:f64,
    y:f64,
}

impl Point{
    fn origin() -> Point{
       Point { x: 0.0, y: 0.0 }
    }

    fn new(xx:f64, yy:f64) -> Point{
        Point { x: xx, y: yy }
    }
}

struct Rectangle{
    p1:Point,
    p2:Point,
}

impl Rectangle{
    //area and peremeter

    fn area(&self) -> f64{
        
        let Point { x:x1, y:y1 } = self.p1;
        let Point { x:x2, y:y2 } = self.p2;
         
        (x2-x1).abs() * (y2-y1).abs()
    }

    fn perimeter(&self) -> f64{
        
        let Point { x:x1, y:y1 } = self.p1;
        let Point { x:x2, y:y2 } = self.p2;
         
        2.0*((x2-x1).abs() + (y2-y1).abs())
        
    }
}

pub fn methods_fun(){
    println!("hheihe");
    let rectangle = Rectangle{
        p1:Point::origin(),
        p2:Point::new(2.4,3.5 ),
    };

    println!(" rectangle perimeter : {:?} and rectangle area: {:?} ",rectangle.perimeter(),rectangle.area());


}