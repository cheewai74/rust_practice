// factorial function
fn factorial(n:i64) -> i64{
    if n == 0 || n == 1 {
        return 1;
    }
    return n*factorial(n-1);
}

// main function
fn main(){

    // call my function
    println!("2! = {}", factorial(2));
    println!("3! = {}", factorial(3));
    println!("4! = {}", factorial(4));
    println!("5! = {}", factorial(5));
    println!("6! = {}", factorial(6));
    println!("7! = {}", factorial(7));
    println!("8! = {}", factorial(8));
    println!("9! = {}", factorial(9));

}