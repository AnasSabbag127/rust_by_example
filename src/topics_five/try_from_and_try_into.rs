use std::convert::TryFrom;

#[derive(Debug,PartialEq)]
struct EvenNumber(i32);


impl TryFrom<i32> for EvenNumber{
    type Error = ();  //..

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value%2 == 0 {
            Ok(EvenNumber(value))
        }
        else {
            Err(())
        }
    }
}

pub fn try_from_and_try_into_fun(){
    // TryFrom
    println!("its running ...");
    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));
    // let num = EvenNumber(34);
    println!("num value : {:?} ",EvenNumber::try_from(341));

    // TryInto

    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));
}