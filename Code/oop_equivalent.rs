/**

Using Struct, Traits and Impl to mimic classes and create objects in rust.

Struct
Traits
Impl 

**/

// -- Struct are user defined types that stores attributes/state of the object --

// Note: Default is private
// struct Cylinder{
//     radius: f32,
//     height: f32,
//     unit: String,
// }

// -- Using pup key to set it as public
// -- radius, height, and unit is private. If we try to access the attribute in another file
pub struct Cylinder{
    radius: f32,
    height: f32,
    unit: String,
}

// -- Traits only holds the behaviour/method of an object. --
// -- Similar to Interface in other programming languages. -- 
// -- Inheritance: To implement has a relationship, Rust uses trait.
trait Formula{
    fn get_area(&self) -> f32;
    fn get_perimeter(&self) -> f32;
}

// -- Is used to add method for struct. 
// -- impl <trait> for <struct>. 


const PI: f32 = 3.1415;

impl Formula for Cylinder{

    fn get_area(&self) -> f32{
        2.0 * PI * self.radius * (self.height + self.radius)
    }

    fn get_perimeter(&self) -> f32{
        2.0 * PI * self.radius
    }
}

// impl Cylinder {

//     // Constructor to cylinder struct
//     fn new(radius: f32, height: f32) -> Cylinder{
//         Cylinder {
//             radius: radius,
//             height: height,
//             unit: "cm".to_string(),
//         }
//     }

//     fn get_summary(&self){
//         println!("Get Summary");
//         println!("r:{}{} X h:{}{}", self.unit, self.height, self.unit, self.height);
//         println!("area:{}\nperimeter:{}", self.get_area(), self.get_perimeter());
//     }
// }

//Implement setter and getter methods give read and write access to attribute
impl Cylinder{

    // Constructor to cylinder struct
    pub fn new(radius: f32, height: f32) -> Cylinder{
        Cylinder {
            radius: radius,
            height: height,
            unit: "cm".to_string(),
        }
    }

    pub fn get_unit(&self) -> &String{
        &self.unit
    }

    pub fn set_unit(&mut self, unit: String){
        self.unit = unit;
    }

    fn get_summary(&self){
        println!("Get Summary");
        println!("r:{}{} X h:{}{}", self.unit, self.height, self.unit, self.height);
        println!("area:{}\nperimeter:{}", self.get_area(), self.get_perimeter());
    }

}

fn main(){

    // // Access attributes by instantiating struct
    // let cylinder = Cylinder{
    //     radius: 1.0,
    //     height: 2.0,
    //     unit: "cm".to_string(),
    // };

    // println!("Radius of cyclinder {} {}", cylinder.radius, cylinder.unit);
    // println!("Height of cyclinder {} {}", cylinder.height, cylinder.unit);

    let cyclinder = Cylinder::new(1.0, 2.0);
    cyclinder.get_summary();
}