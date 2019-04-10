
extern crate futures;

use futures::future;
use futures::prelude::*;
use std::thread;
use std::time;

pub fn f_main() {
    let future1 = future::lazy(|| {
        thread::sleep(time::Duration::from_secs(2));
        future::ok::<char, ()>('a')
    });

    let future2 = future::lazy(|| {
        thread::sleep(time::Duration::from_secs(1));
        future::ok::<char, ()>('b')
    });

    let (value, last_future) = future1.select(future2).wait().ok().unwrap();
    assert_eq!(value, 'a');
    assert_eq!(last_future.wait().unwrap(), 'b');
}
