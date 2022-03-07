use std::any;
use std::fmt;

// fn print_type<T: std::fmt::Display>(item: T){
//     println!("{} is {}", item, any::type_name::<T>());
// }

fn print_type<T: std::fmt::Debug>(item: T){
    println!("{:?} is {}", item, any::type_name::<T>());
}

fn main(){
    print_type(13);
    print_type(13.0);
    print_type("thirteen");

    // Using Debug format
    print_type([13]);
}

