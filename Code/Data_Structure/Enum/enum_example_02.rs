#[derive(Debug)]
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64, f64)
}

impl Shape{

    fn get_perimeter(&self) -> f64 {
        match *self{
            Shape::Circle(r)=> r * 2.0 * std::f64::consts::PI,
            Shape::Rectangle(w, h) => (2.0 * w) + (2.0 * h),
            Shape::Triangle(a, b, c) => a + b + c
        }
    }
}

fn main(){
    let my_shape = Shape::Rectangle(1.2, 3.4);
    println!("my_shape is {:?}", my_shape);

    // match my_shape{
    //     Shape::Circle(r) => println!("Circle with radius {}", r),
    //     Shape::Rectangle(w, h) => println!("{} x {} Rectangle", w, h),
    //     Shape::Triangle(a, b, c) => println!("Triangle with sides {}, {}, {}", a, b, c)
    // }

    let perimeter = my_shape.get_perimeter();
    println!("perimeter is {}", perimeter);

}