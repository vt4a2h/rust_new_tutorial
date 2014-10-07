fn main() {
	let x = 5i;

	if x == 5i {
		println!("x is five!");
	} else {
		println!("x is not five :(");
	}

	// if is expression (returns value)
	let y = if x == 5i { 10i } else { 15i };
	println!("y = {}", y);
}