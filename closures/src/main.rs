fn main() {
    let add_one = |x| { 1i + x };
    println!("5 + 1 = {}", add_one(5i));

    let x = 5i;
    // lambda
    let printer = || { println!("x is {}", x); }; // x borrowed now
    printer();

    let p = proc() { x * x }; // can be called once
    println!("{}", p());

    let square = |x: int| { x * x };
    let result = twice(5i, square);
    println!("{}", result);
}

fn twice(x: int, f: |int| -> int) -> int {
	f(x) + f(x)
}