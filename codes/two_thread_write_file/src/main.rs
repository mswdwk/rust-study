use std::thread;
use std::path::Path;
use std::fs::{OpenOptions,File};
use std::io::Error;
use std::io::Write;

type Result<T> = std::result::Result<T,Error>;
pub fn append_file_data(path:&str,data: &[u8]) -> Result<()> {
  let c = || -> Result<()> {  
	let mut file = if Path::new(path).exists() {
        // append data
        OpenOptions::new().append(true).open(path)?
    } else {
      println!("create file: path= {} {:?}", &path,data);
        File::create(&path)?
    };
    let mut written_len=0; 
	for _ in 0..20 {
        written_len += file.write(data)?;
    }
    println!("append file data {}",written_len );
	 Ok(())
	};
	if let Err(err) = c() {
	    println!("thread_1: err= {:?}",err);
	}
	Ok(())
}

fn main() {
    thread::spawn(||{
	    append_file_data("test.txt",&[65,65,65,65])
    });
    thread::spawn(||{
        append_file_data("test.txt",&[66,66,66,66])
    });

    thread::sleep(std::time::Duration::from_secs(1));
    println!("Hello, world!");
}
