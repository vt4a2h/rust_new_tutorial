fn main() {
	// by default, bindings are immutable
	let x = 5i; // simple bindings
	let (y, z) = (1i, 2i); // pattern bindings
	let a: int = 5; // with type
	let b: int; // without init, cannot be printed

	let mut c = 5i; // can be changed
	c = 10i;

	println!("x = {}\ny = {}\nz = {}\na = {}\nc = {}", x, y, z, a, c);
}