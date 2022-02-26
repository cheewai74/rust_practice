/**

Using Struct, Traits and Impl to mimic classes and create objects in rust.

Struct
Traits
Impl 

**/

// -- Struct are user defined types that stores attributes/state of the object --
struct Cylinder{
    radius: f32,
    height: f32,
    unit: String,
}

// -- Traits only holds the behaviour/method of an object. --
// -- Similar to Interface in other programming languages. -- 
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

impl Cylinder {

    // Constructor to cylinder struct
    fn new(radius: f32, height: f32) -> Cylinder{
        Cylinder {
            radius: radius,
            height: height,
            unit: "cm".to_string(),
        }
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