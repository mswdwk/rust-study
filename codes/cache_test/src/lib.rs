pub mod cache;
pub mod errors;

#[macro_use]
extern crate error_chain;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
