use std::vec;
fn calc(channels: &mut Vec<i32>)
{
    println!("channels= {:?}",channels);
    channels.retain(| c|*c != 2);
    // channels.remove(1);
    // channels.insert(5,12); // painc
    channels.push(6);
    println!("after channels= {:?}",channels);

    println!("aa after channels= {:?}",channels);
}
fn main()
{
    let mut channels = vec![1,2,3];
    calc(&mut channels);
    println!("after 2 channels= {:?}",channels);
    channels = vec![1,9,5];
    println!("after 3 channels= {:?}",channels);
}