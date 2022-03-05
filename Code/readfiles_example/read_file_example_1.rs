use std::fs;

fn main(){

    // Use unwrap funtion to get it's valuees
    let contents = fs::read_to_string("planets.txt").unwrap();
    println!("contents is {}\n\n", contents);

    for line in contents.lines(){
        println!("line is {}", line)
    }

    // Display the byte of individual file.
    let contents = fs::read("planets.txt").unwrap();
    println!("contents is {:?}\n\n", contents);

}