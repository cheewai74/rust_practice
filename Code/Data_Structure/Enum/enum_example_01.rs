/**
 * enum EnumValue{
 *      Value1(optional_type),
 *      Value2(optional_type),
 *      Value3(optionla_type)
 * }
 * 
 */

 // To initialize an enum with values, 
 // we assign the enum with a value to a variable.

 #[derive(Debug)]
 enum Gender{
     Male(i32),
     Female(i32),
 }

 fn main(){

    let male = Gender::Male(0);
    let female = Gender::Female(1);

    println!("Male {:?}", male);
    println!("Female {:?}", female);
 }