#[allow(unused)]
pub fn let_else_fun(){
    let string = "hello there";
    let mut it = string.split(" ");

    it.next();// after this it point on 2nd word 
    let (Some(word1),Some(word2) ) = (it.next(),it.next())else{
        panic!("not match with words ");
    };

}


