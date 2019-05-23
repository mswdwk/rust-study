use crate::errors::*;
use mempool;
pub trait Cache<Key,Value>
where  Key: ::std::cmp::Eq + ::std::hash::Hash + Clone
{
    // type Key = key;
    // type Value = value;
    fn get(key: Key) -> Value;
    fn create(key:Key,value:Value) -> Result<()>;
    fn delete(key:Key) -> Result<()>;
    fn update(key:Key) -> Result<()>;

    fn create_key(key:Key) -> String;
}

struct CacheMemoryPool{
    pub bytes_size: i64,
    pub entry_num: i32,
    pub max_size: i64,
}
