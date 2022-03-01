/**
 * Access tuple values
 */

 fn main(){

        // Create a tuple
        let my_tuple:(i32, f64, &str) = (100, 3.14, "Mark");

        // Show access tuble elements
        println!("{:?}", my_tuple.0);
        println!("{:?}", my_tuple.1);
        println!("{:?}", my_tuple.2);
 }