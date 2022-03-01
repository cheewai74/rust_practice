fn main(){

    // Create an array
    let arr = [10, 20, 30, 40, 50, 60, 70];

    // for loop
    for i in 0..arr.len(){
        println!("arr[{}] = {}", i, arr[i]);
    }

    // break
    println!("\n\n");
    for i in 0..arr.len(){
        if i == 3{
            break;
        }
        println!("arr[{}] = {}", i, arr[i]);
    }

    // continue
    println!("\n\n");
    for i in 0..arr.len(){
        if i == 3{
            continue;
        }
        println!("arr[{}] = {}", i, arr[i]);
    }

}