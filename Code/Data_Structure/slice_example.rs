// slice syntax
// &var_name[start_index..end_index];

fn main(){

    let msg = "Hello, How are you";
    println!("Pre-slice (msg): {}", msg);

    let my_slice = &msg[0..10];
    println!("Post-slice (my_slice): {}", my_slice)
}