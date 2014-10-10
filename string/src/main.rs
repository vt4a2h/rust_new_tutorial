fn main() {
	let string = "Hi!"; // string slice str&, fixed size

	let mut s = "Hello".to_string(); // in-memory string
	println!("{}", s);

	s.push_str(", world!");
	println!("{}", s);

	let hi = "Hi!".to_string();
	takes_slice(hi.as_slice());

	// String -> &str -- cheaper than &str -> String
	// use string.as_slice() == "some string"
}

fn takes_slice(slice: &str) {
	println!("Got: {}", slice);
}