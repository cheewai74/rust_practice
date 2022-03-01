/**
 * 
 *  while boolean_conditions {
 *      // code if condition is true.
 *  }
 * 
 */

 fn main(){

    // Create an array
    let arr = [10, 20, 30, 40, 50, 60, 70];

    let mut i = 0;

    // while loop
    while i < 7 {
        println!("arr[{}] = {}", i, arr[i]);
        i = i + 1;
    }

 }