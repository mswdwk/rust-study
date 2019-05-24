pub mod errors;
pub mod types;
pub mod store;

// #[macro_use]
// extern crate error_chain;


#[cfg(test)]
mod tests{
    use super::types::*;
    use super::store::*;
    use ::std::mem::transmute;

    #[test]
    fn test_cache(){
        let mut store = MemStore::new();
        let key: Vec<u8> = vec![1,2,3];
        let value: Vec<u8> = vec![4,5,6];
        store.set(&key,value);
        let res = store.get(&key);
        println!("ok! res= {:?}",res);
    }

    impl MemcachedKeyTrait for i32 {
        fn into_mck(&self) -> MemcachedKey {
            unsafe { transmute::<i32,[u8;4]>(*self).to_vec() }
        }
    }

    impl MemcachedKeyTrait for String {
        fn into_mck(&self) -> MemcachedKey {
            let tmp = self.clone();
            tmp.into_bytes()
        }
    }

    impl MemcachedKeyTrait for &str {
        fn into_mck(&self) -> MemcachedKey {
            let tmp = self.to_string();
            tmp.into_bytes()
        }
    }

    impl MemcachedValueTrait for i32 {
        fn into_mcv(&self) -> MemcachedKey {
            unsafe { transmute::<i32,[u8;4]>(*self).to_vec() }
        }
    }

    impl MemcachedValueTrait for &str {
        fn into_mcv(&self) -> MemcachedValue {
            let tmp = self.to_string();
            tmp.into_bytes()
        }
    }

    #[test]
    fn test_from_i32(){
        let mut store = MemStore::new();
        let key= 64.into_mck();
        let value: Vec<u8> = vec![4,5,6];
        store.set(&key,value);
        let res = store.get(&key);
        println!("ok! res= {:?}",res);
    }

    #[test]
    fn test_from_str(){
        let mut cache_sotre = { MemStore::new()};

        let key= "64".into_mck();
        let value = "value".into_mcv();
        cache_sotre.set(&key,value);
        let res = cache_sotre.get(&key);
        println!("ok! res= {:?}",res);
    }

    #[test]
    fn test_from_str_expire() {
        let mut cache_sotre = { MemStore::new()};

        let key= "64".into_mck();
        let value = "value".into_mcv();
        cache_sotre.set(&key,value);
        let res = cache_sotre.get(&key);
        println!("ok! res= {:?}",res);
    }
}

