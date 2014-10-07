fn main() {
	print_number(10);
	sum(10, 15);

	let x = inc(10);
	print_number(x);

	let y = inc_2(3);
	print_number(y);
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