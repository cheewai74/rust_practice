/**
 *  Create a function with return
 * 
 *  fn function_name() -> return_type{
 *      // Statements
 *      return value;
 *  }
 * 
 */

 // Create a function with return
 fn my_value()  -> i64 {
     // This funcion will return 10
     return 10;
 }

 // main function
 fn main() {

    // call my function
    let x = my_value();
    println!("The value of x equal: {}", x);
 }
