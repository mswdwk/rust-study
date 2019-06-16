use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let mutex = Arc::new(Mutex::new(HashSet::new()));

    // poison the mutex
    let c_mutex = mutex.clone();
    let _ = thread::spawn(move || {
     let mut data = c_mutex.lock().unwrap();
     data.insert(10);
     panic!();
    }).join();

    let p_err = mutex.lock().unwrap_err();
    let data = p_err.into_inner();
    println!("recovered {} items", data.len());
    println!("data content {:?} ", data);
}