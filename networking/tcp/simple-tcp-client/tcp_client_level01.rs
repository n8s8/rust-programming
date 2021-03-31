use std::net::{ TcpStream, Shutdown };
use std::io::{ BufRead, BufReader };
use std::str;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8888")
        .expect("Error connecting to the server");
    println!("Connected to server");
    let mut buffer: Vec<u8> = Vec::new();
    let mut reader = BufReader::new(&stream);
    reader.read_until(b'\n', &mut buffer)
        .expect("Error reading into buffer");
    print!("{}", str::from_utf8(&buffer).expect("Error writing buffer as string"));
    stream.shutdown(Shutdown::Both).expect("Shutdown failed");
}