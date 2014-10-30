use std::sync::Future;
use std::task;
use std::rand;

fn main() {

	// simple task
	// lightweight, doesn't share memory
    spawn(proc() {
    	println!("Hi!");
    });

    // channels for send data between tasks
    let (tx, rx) = channel();
    spawn(proc() {
    	tx.send("Hello from the task!".to_string());
    });

    // .try_recv() returns Result<T, TryRecvError>
    let message = rx.recv();
    println!("{}", message);

    let (tx1, rx1) = channel();
	let (tx2, rx2) = channel();

	spawn(proc() {
		tx1.send("Hello from the task!".to_string());
		let message = rx2.recv();
		println!("{}", message);
	});

	let message = rx1.recv();
	println!("{}", message);

	tx2.send("Goodbye from main!".to_string());

	// futures
	// must be mutable
	let mut delayed_value = Future::spawn(proc() {
		12345i // example
	});
	println!("value = {}", delayed_value.get())

	// handle fails
	let result = task::try(proc() {
		if rand::random() {
			println!("OK!");
		} else {
			fail!("ooops!");
		}
	});
}
