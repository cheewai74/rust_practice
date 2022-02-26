// Polymorphism - is an ability that allows single interface for multiple types.

const PI: f32 = 3.1415;

trait Formula {
    fn get_area(&self) -> f32;
}

struct Cylinder{
    radius: f32,
    height: f32,
}

struct Triangle {
    height: f32,
    base: f32,
}

impl Formula for Cylinder{
    fn get_area(&self) ->f32{
        2.0 * PI * self.radius * (self.radius + self.height)
    }
}

impl Formula for Triangle{
    fn get_area(&self) -> f32{
        0.5 * PI * self.base + self.height
    }
}

fn main(){
    let cylinder = Cylinder{radius: 1.0, height: 2.0};
    let triangle = Triangle{base: 1.0, height: 2.0};
    println!("area of cylinder {}", cylinder.get_area());
    println!("area of triangle {}", triangle.get_area());
}