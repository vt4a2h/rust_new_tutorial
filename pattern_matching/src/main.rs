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
}

fn cmp(a: int, b: int) -> Ordering {
	if a < b { Less }
	else if a > b { Greater }
	else { Equal }
}