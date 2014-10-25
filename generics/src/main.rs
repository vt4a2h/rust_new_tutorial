fn main() {
    let a: Option<int> = Some(5i);
    let b: Option<f64> = Some(5.0f64);
    let c: Result<f64, String> = Ok(2.3f64);
    let d: Result<f64, String> = Err("There was na error".to_string());

    let e = inverse(25.0f64);

    match e {
    	Ok(e)    => println!("The inverse of 25 is {}", e),
    	Err(msg) => println!("Error: {}", msg),
    }
}

fn inverse<T>(x: T) -> Result<T, String> {
	if x == 0.0 { return Err("x cannot be zero!".to_string()); }

	Ok(1.0 / x)
}

// traits
struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

impl HasArea for int {
    fn area(&self) -> f64 {
        println!("this is silly");

        *self as f64
    }
}

// Any types with method with HasArea impl
fn print_area<T: HasArea>(shape: T) {
    println!("This shape has an area of {}", shape.area());
}

// exampel
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

// right way
// use shapes::HasArea;

// mod shapes {
//     use std::f64::consts;

//     pub trait HasArea {
//         fn area(&self) -> f64;
//     }

//     pub struct Circle {
//         pub x: f64,
//         pub y: f64,
//         pub radius: f64,
//     }

//     impl HasArea for Circle {
//         fn area(&self) -> f64 {
//             consts::PI * (self.radius * self.radius)
//         }
//     }
// }


// fn main() {
//     let c = shapes::Circle {
//         x: 0.0f64,
//         y: 0.0f64,
//         radius: 1.0f64,
//     };

//     println!("{}", c.area());
// }