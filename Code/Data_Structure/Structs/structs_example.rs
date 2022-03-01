/**
 *  Definition:
 * 
 *  struct struct_name{
 *      property_name:  data_type,
 *      property_name:  data_type,
 *      property_name:  data_type    
 *  }
 * 
 *  Initialization:
 * 
 *  let variable_name = struct_name{
 *      property_name: property_value,
 *      property_name: property_value,
 *      property_name: property_value
 *  };
 * 
 *  Access:
 *  variable_name.property_name;
 * 
 */

 // Create Structure
struct Car{
    name:String, 
    color:String,
    speed:i64
}

fn main(){

    // Structure initialization
    // Note: Added mut to allow struct to be mutable. 
    let mut my_car = Car{
        name:String::from("Tesla"),
        color:String::from("Black"),
        speed:500
    };

    // How to access struct properties
    println!("Car Name is :{}", my_car.name);
    println!("Car Color is :{}", my_car.color);
    println!("Car Speed is :{}", my_car.speed);

    println!("=========================================================");

    // Change struct property values.
    // Note: mut keyword in Struct Init

    my_car.name = String::from("Ferrari");
    my_car.color = String::from("Red");
    my_car.speed = 400;

    println!("Car Name is :{}", my_car.name);
    println!("Car Color is :{}", my_car.color);
    println!("Car Speed is :{}", my_car.speed);

}