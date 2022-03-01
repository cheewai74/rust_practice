/**
 * 
 *  fn function_name(arg_1, arg_2....,arg_N) -> return_type{
 *      // statements
 *      return value;
 *  }
 * 
 */

 // Create a function with arguments
 fn sum(x:i64, y:i64) -> i64{
     return x+y;
 }

 // main function
 fn main(){

    // Call my function
    let my_sum = sum(100, 405);

    println!("x + y = {}", my_sum);
 }