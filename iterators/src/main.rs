fn main() {
    let mut range = range(0i, 10i);

    loop {
    	match range.next() {
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

    // consumers
    let greatter_than_forty_two = range(0i, 100i).find(|x| *x >= 42);
    match greatter_than_forty_two {
    	Some(_) => println!("We got some numbers!"),
    	None    => println!("No numbers found :("),
    }
}
