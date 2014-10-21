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
}
