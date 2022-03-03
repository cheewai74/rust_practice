fn main(){

    let mut stuff:(u8, f32, char) = (10, 3.14, 'x');
    let first_item = stuff.0;
    println!("first_item is {}", first_item);
    stuff.0 +=3;
    // let first_item = stuff.0;
    println!("first_item after increment is {}", first_item);

    let (a, b, c) = stuff;
    println!("b is {}", b);

}