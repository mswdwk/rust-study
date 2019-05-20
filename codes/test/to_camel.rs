pub fn to_camel(s: &str) -> String {
    let s = s.split('_').map(|s| {
        let mut c = s.chars();
        match c.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + c.as_str()
        }
    });

    use std::iter::FromIterator;
    String::from_iter(s)
}

fn main() {
  let str1= "abc_123_hello_";
  let str2= "_abc_123_hello";
  let str3= "abc_123_hello";
  println!("{} to_camel {}",str1,to_camel(str1));
  println!("{} to_camel {}",str2,to_camel(str2));
  println!("{} to_camel {}",str3,to_camel(str3));
}

