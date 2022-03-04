/**
 * 1. When using a mutable reference, you cannot create other references.
 * 2. Prevent data races.
 * 3. let ref1 = &mut var;
*/
fn main() {

    let mut rocket_fuel = String::from("RP-1");
    let length = process_fuel(&mut rocket_fuel);
    println!("rocket_fuel is {} and length is {}", rocket_fuel, length);

}

fn process_fuel(propellant: &mut String) ->  usize {
    println!("processing propellant {}...", propellant);
    let length = propellant.len();
    length
}