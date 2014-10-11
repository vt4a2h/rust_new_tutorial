use std::io::stdin;

fn main() {
	println!("Type something!");

	let input = stdin()
	                .read_line()
	                .ok()
	                .expect("Faild to read line");

	println!("{}", input);
}