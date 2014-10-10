fn main() {
	let v1 = vec![1i, 2i, 3i]; // array with dynamic size
	let v2 = [1i, 2i, 3i];     // size fixed
	let v3 = [1i, ..20];       // size fixed, 20 elements init by 1

	let slice = v1.as_slice(); // slice &[T]

	for i in v1.iter() {
		println!("{}", i);
	}
}