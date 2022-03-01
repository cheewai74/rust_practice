fn main(){

    // Create a string
    let mut my_string = String::from("Hello ");

    // add a char to my string
    my_string.push('w');
    my_string.push_str("orld!");

    // show string
    println!("{}", my_string);

}