fn main() {
	print_number(10);
	sum(10, 15);

	let x = inc(10);
	print_number(x);

	let y = inc_2(3);
	print_number(y);

	let c = Circle { x: 0.0, y: 0.0, radius: 2.0 };
	println!("{}", c.area());

	let c1 = Circle::new(0.0, 0.0, 2.0);
}

struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle {
	// self, &self, &mut self
	fn area(&self) -> f64 {
		std::f64::consts::PI * (self.radius * self.radius)
	}

	// static method
	fn new(x: f64, y: f64, radius: f64) -> Circle {
		Circle {
			x: x,
			y: y,
			radius: radius,
		}
	}
}

fn print_number(x: int) {
	println!("value is: {}", x);
}

fn sum(x : int, y : int) {
	println!("sum is: {}", x + y);
}

fn inc(x: int) -> int {
	x + 1
}

fn inc_2(x : int) -> int {
	if x < 5 { return x; }

	x + 1
}