use std::fs::File;
use tracing::{error, info};
use tracing_subscriber;
use std::io;
use std::io::Read;
fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn main() {
   let res =  read_username_from_file();
    match res {
        Ok(val) => println!("{}",val),
        Err(err) => println!("{}",err)
    }
}