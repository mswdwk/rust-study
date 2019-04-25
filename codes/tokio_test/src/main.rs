extern crate http;
mod http_test;
use http_test::*;

fn main() {
    println!("Hello, world!");
    http_header_eq_test_main();
    http_header_ne_test_main();
}
