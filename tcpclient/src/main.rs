use std::io::{Read,Write};
use std::net::TcpStream;
use std::str;


fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:3000").unwrap();
    stream.write("Hello World!".as_bytes()).unwrap();
    // let mut buffer= Vec::with_capacity(5);
    let mut buffer= [0;100];
    stream.read(&mut buffer).unwrap();
    println!("Response from server:{:?}",String::from_utf8_lossy(&buffer));
}
