fn main() {
    let mut range1 = range(0i, 10i);

    loop {
    	match range1.next() {
    		Some(x) => {
    			println!("{}", x);
    		},
    		None => { break }
    	}
    }

    let nums = vec![1i, 2i, 3i];
    for num in nums.iter() {
    	println!("{}", *num);
    }

    let one_two = range(1i, 2i).collect::<Vec<int>>();

    // consumers
    let greatter_than_forty_two = range(0i, 100i).find(|x| *x >= 42);
    match greatter_than_forty_two {
    	Some(_) => println!("We got some numbers!"),
    	None    => println!("No numbers found :("),
    }

    let sum = range(1i, 4i).fold(0i, |sum, x| sum + x);
    println!("{}", sum);
  
    // iterators are lazy
    range(1i, 100i).map(|x| x + 1i);            // no effect and warning
    range(1i, 100i).map(|x| println!("{}", x)); // the same with previous

    for i in std::iter::count(0i, 1i).take(5) {
    	println!("{}", i);
    }

    for i in range(1i, 100i).filter(|x| x % 2 == 0) {
    	println!("{}", i);
    }

    let ve1 = range(1i, 1000i)
              .filter(|x| x % 2 == 0)
              .filter(|x| x % 3 == 0)
              .take(5)
              .collect::<Vec<int>>();

    for element in ve1.iter() {
    	println!("{}", element);
    }
}
