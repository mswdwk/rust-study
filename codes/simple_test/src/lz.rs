// use crate::lazy_static;
use crate::HASHMAP;
use std::collections::HashMap;


lazy_static! {
    pub static ref HASHMAP_2: i32 = {
        init_lazy()
    };
}

pub fn print_map(amap: &HashMap<u32,&'static str>) {
for (k,v) in amap.iter() {
        println!("k= {:?},v= {:?}",k,v);
    }
}

fn init_lazy() -> i32{
    let mut map = HASHMAP.write().unwrap();
    println!("before");
    print_map(&*map);
    println!("after ");
    map.insert(3,"123");
    // print_map(&*map);
    println!("init lzay in lz");
    10
}