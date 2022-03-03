fn main(){

    let message = ['h','e','l','l','o'];

    for (index, &item) in message.iter().enumerate(){
        println!("item {} is {}", index, item);
        if item == 'e'{
            break;
        }
    }
}