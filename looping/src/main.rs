fn main() {
	for x in range(0i, 10i) {
		println!("{:d}", x);
	}

	let mut x = 5u;
	let mut done = false;

	while !done {
		x += x - 3;
		println!("{}", x);
		if x % 5 == 0 { done = true; }
	}

	// infinite loop
	// while true { ... } or loop { ... }

	// "continue" and "break" work for loops
}