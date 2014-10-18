enum OptionalInt {
	Value(int),
	Missing,
}

fn main() {
	let x = 5i;

	match x {
		1 => println!("one"),
		2 => println!("two"),
		_ => println!("smth else"),
	}

	let y = 5i;
	let z = 10i;

	match cmp(z, y) { 					// let result = match ...
		Less    => println!("less"),
		Greater => println!("greator"),
		Equal   => println!("equal"),
	}

	let a = 1i;
	match a {
		1 | 2 => println!("one or two"),
		_     => println!("anything"),
	}

	let b = 4i;
	match b {
		1 ... 5 => println!("int ramge [1..5]"),
		_       => println!("anything"),
	}

	let c = 1i;
	match c {
		c @ 1 ... 5 => println!("got {}", c), // bind value
		_           => println!("anything"),
	}

	let d = Value(6i);
	match d {
		Value(x) if x > 5 => println!("biggre than five."),
		Value(..)         => println!("got an int"), // ignore value
		Missing           => println!("No such luck."),
	}
}

fn cmp(a: int, b: int) -> Ordering {
	if a < b { Less }
	else if a > b { Greater }
	else { Equal }
}