#[derive(Debug)]
#[derive(Clone)]

struct Shuttle{
    name: String,
    crew_size: u8,
    propellant: f64
}

impl Shuttle{
    fn get_name(&self) -> &str {
        &self.name
    }

    fn add_fuel(&mut self, gallons: f64){
        self.propellant += gallons;
    }

    // Associated functions
    fn new(name: &str) -> Shuttle{

        Shuttle{
            name: String::from(name),
            crew_size: 7,
            propellant: 0.0
        }

    }
}

fn main(){

    // let mut vehicle = Shuttle{
    //     name: String::from("Endeavour"),
    //     crew_size: 7,
    //     // propellant: 835958.0
    //     propellant: 0.0
    // };

    let mut vehicle = Shuttle::new("Endeavour");
    let mut vehicle2 = Shuttle::new("Discovery");

    let vehicle_name = vehicle.get_name();
    println!("vehicle_name is {}", vehicle_name);

    let vehicle2 = Shuttle{
        // name: String::from("Discovery"),
        // ..vehicle
        ..vehicle.clone()
    };

    // println!("name is {}", vehicle.name);
    // println!("crew size is {}", vehicle.crew_size);
    // println!("propellant is {}\n\n", vehicle.propellant);

    vehicle.name = String::from("Atlantis");
    println!("vehile is {:?}", vehicle);
    println!("vehile is {:?}", vehicle2);

    println!("propellant is {}", vehicle.propellant);
    vehicle.add_fuel(1000.0);
    println!("propellant is {}", vehicle.propellant);


}