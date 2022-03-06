#[derive(Debug)]

struct Shuttle{
    name: String,
    crew_size: u8,
    propellant: f64
}

fn main(){
    let mut vehicle = Shuttle{
        name: String::from("Endeavour"),
        crew_size: 7,
        propellant: 835958.0
    };

    println!("name is {}", vehicle.name);
    println!("crew size is {}", vehicle.crew_size);
    println!("propellant is {}\n\n", vehicle.propellant);

    vehicle.name = String::from("Atlantis");
    println!("vehile is {:?}", vehicle);

}