use std::{sync::{Mutex, Arc}, thread};

pub fn run() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    (0..10).for_each(|_| {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });

        handles.push(handle);
    });

    handles.into_iter().for_each(|h| h.join().unwrap());

    println!("Result: {}", *counter.lock().unwrap());
}
