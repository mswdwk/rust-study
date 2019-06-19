use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

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
    
	  let rw = std::sync::RwLock::new(123);
	  
    let now = Instant::now();
    let loop_time = 10_000_000;
    for _ in  0..loop_time {
       let _ = rw.read();
    }
    let escp  = now.elapsed().as_nanos();
    let per = escp / loop_time;
    println!("rwlock total cost= {}ms once_cost= {}ns",now.elapsed().as_millis(),per  );
    
    let now = Instant::now();
    let loop_time = 10_000_000;
    for i in  0..loop_time {
       let _ = i;
    }
    let escp  = now.elapsed().as_nanos();
    let per = escp / loop_time;
    println!("read mem total cost= {}ms once_cost= {}ns",now.elapsed().as_millis(),per  );
    
    let now = Instant::now();
    for _ in  0..loop_time {
       let _ = now.elapsed().as_secs();
    }
    let escp  = now.elapsed().as_nanos();
    let per = escp / loop_time;
    println!("read Instant total cost= {}ms once_cost= {}ns",now.elapsed().as_millis(),per  );

}
