fn main() {
	let x = (1i, "hello"); // tuple
	let x1 : (int, &str) = (1, "hello"); // with type annotated

	let (a, b, c) = (1i, 2i, 3i);
	println!("a is {}", a);

	let d = (1i, 2i ,3i);
	let e = (2i, 3i, 4i);

	 if d == e { 
	 	println!("yes")
	 } else { 
	 	println!("no")
	 }

	 let (f, g) = next_two(5i);
	 println!("f, g = {}, {}", f, g);

	 // struct
	 // arbitrary order initialisation
	 let origin = Point { x: 0i, y: 0i };
	 println!("The origin is at ({}, {})", origin.x, origin.y);

	 let mut point = Point { x: 0i, y: 0i };
	 point.x = 5;
	 println!("The point is at ({}, {})", point.x, point.y);

	 // tuple struct -- original rust type
	 let black = Color(0, 0, 0);
	 // use like type alias
	 struct Inches(int);
	 let length = Inches(10);
	 let Inches(integer_length) = length;
	 println!("length is {} inches", integer_length);

	 // enum
	 let h = 5i;
	 let i = 10i;

	 let ordering = cmp(h, i);

	 if ordering == Less {
	 	println!("less");
	 } else if ordering == Greater {
	 	println!("greater");
	 } else if ordering == Equal {
	 	println!("equal");
	 }

	 // enum with values
	 let k = Value(5);
	 let l = Missing;

	 match k {
	 	Value(n) => println!("x is {:d}", n),
	 	Missing  => println!("x is missing!"),
	 }

	 match l {
	 	Value(n) => println!("y is {:d}", n),
	 	Missing  => println!("y is missing!"),
	 }
}

fn next_two(x: int) -> (int, int) { (x + 1i, x + 2i) }

struct Point {
    x: int,
    y: int,
}

struct Color(int, int, int);

fn cmp(a: int, b: int) -> Ordering {
	if a < b { Less }
	else if a > b { Greater }
	else { Equal }
}

// enum with values
enum OptionalInt {
	Value(int),
	Missing, 
}

// you can have any numbers of values
enum OptionalColor {
	Color1(int, int, int),
	Missing1,
}