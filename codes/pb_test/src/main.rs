pub mod idl;
pub mod fut;
use self::idl::content::{ContentInt, ContentStr, ContentStr4};
use protobuf::error::*;
use protobuf::{parse_from_bytes, Message};

fn test_parse_pbdata_with_field_type_different() -> Result<(), ProtobufError> {
    let mut content_int = ContentInt::new();
    content_int.set_one(true);
    content_int.set_two(1);
    content_int.set_three("hello".to_string());

    let bytes = content_int.write_to_bytes()?;
    let cont_str = parse_from_bytes::<ContentStr>(&bytes)?;
    println!("one= {:?}", cont_str.get_one());
    println!("two= {:?}", cont_str.get_two());
    println!("three= {:?}", cont_str.get_three());

    Ok(())
}

fn test_parse_pbdata_with_more_fields() -> Result<(), ProtobufError> {
    let mut content_str4 = ContentStr4::new();
    content_str4.set_one(true);
    content_str4.set_two("1".into());
    content_str4.set_three("hello".to_string());
    content_str4.set_four("four".to_owned());

    let bytes = content_str4.write_to_bytes()?;
    let cont_str = parse_from_bytes::<ContentStr>(&bytes)?;
    println!("one= {:?}", cont_str.get_one());
    println!("two= {:?}", cont_str.get_two());
    println!("three= {:?}", cont_str.get_three());

    Ok(())
}

fn test_parse_pbdata_with_less_fields() -> Result<(), ProtobufError> {
    let mut content_str4 = ContentStr::new();
    content_str4.set_one(true);
    content_str4.set_two("1".into());
    content_str4.set_three("hello".to_string());

    let bytes = content_str4.write_to_bytes()?;
    let cont_str = parse_from_bytes::<ContentStr4>(&bytes)?;
    println!("one= {:?}", cont_str.get_one());
    println!("two= {:?}", cont_str.get_two());
    println!("three= {:?}", cont_str.get_three());
    println!("four= {:?}", cont_str.get_four());

    Ok(())
}

fn main() {
    if let Err(err) = test_parse_pbdata_with_field_type_different() {
        println!("err= {:?}", err);
    }
    if let Err(err) = test_parse_pbdata_with_more_fields() {
        println!("err= {:?}", err);
    }
    if let Err(err) = test_parse_pbdata_with_less_fields() {
        println!("err= {:?}", err);
    }
    self::fut::f_main();
}
