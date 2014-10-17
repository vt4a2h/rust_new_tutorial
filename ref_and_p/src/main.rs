fn main() {
    let x = 5i;
    let y = &x; // reference to x, immutable by default
    let z = &x; // you can have multiple references

    // let a = &mut x; // error: x -- immutable

    let mut b = 10i;
    let c = &mut b; // ok. mutable link to mutable data
    // let d = &mut b; // error only one mutable link

    assert_eq!(5i, *y); // dereference
    assert_eq!(6, add_one(&5));
}

fn add_one(x: &int) -> int { *x + 1 }
