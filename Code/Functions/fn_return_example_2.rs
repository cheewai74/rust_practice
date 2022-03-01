// Create a function with return
fn sum() -> i64{
    let x = 100;
    let y = 590;
    return x + y;
}

// main function
fn main(){

    // call my function
    let my_sum = sum();
    println!("x + y = {}", my_sum);
}