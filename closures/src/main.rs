fn main() {
    let add_one = |x| { 1i + x };
    println!("5 + 1 = {}", add_one(5i));

    let x = 5i;
    // lambda
    let printer = || { println!("x is {}", x); }; // x borrowed now
    printer();

    let y = 4i;
    let p = proc() { x * x }; // can be called once
    println!("{}", p());
}
