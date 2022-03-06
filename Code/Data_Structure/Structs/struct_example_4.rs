/**
 * Generic Data Types:
 * Abstract stand-ins for concrete data types or other properties.
 * Can be used with structs, functions, methods and more.
 * Defined with <T>
 */

#[derive(Debug)]

struct Rectangle<T, U>{
// struct Rectangle<T>{    

    // width: f64,
    // height: f64

    width: T,
    // height: T

    height: U
}

impl<T, U> Rectangle<T, U>{
    fn get_width(&self) -> &T{
        &self.width
    }
}

impl Rectangle<u8, u8>{
    fn get_perimeter(&self) -> u8 {
        2 * self.width + 2 * self.height
    }
}

fn main(){

    let rect =Rectangle {
        // width: 1.2,
        // height: 3.4

        // width: 1,
        // height: 3

        width: 1u8,
        height: 3u8
    };

    println!("rect is {:?}", rect);
    println!("width is {}", rect.get_width());
    println!("perimeter is {}", rect.get_perimeter())
}