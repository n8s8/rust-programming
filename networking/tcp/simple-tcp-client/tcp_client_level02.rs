use std::net::{ TcpStream, Shutdown };
use std::str;
use std::io::{ self, BufRead, BufReader, Write };

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

fn main() {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    let mut stream = TcpStream::connect("127.0.0.1:8888")
            .expect("Could not connect to server");

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    loop {
        while running.load(Ordering::SeqCst) {
        let mut input = String::new();
        let mut buffer: Vec<u8> = Vec::new();
        io::stdin().read_line(&mut input)
            .expect("Faiiled to read from stdin");
        stream.write(input.as_bytes())
            .expect("Failed to write to server");
        let mut reader = BufReader::new(&stream);
        reader.read_until(b'\n', &mut buffer)
            .expect("Error reading into buffer");
        print!("{}", str::from_utf8(&buffer).expect("Could not write buffer as string"));
        }
        stream.shutdown(Shutdown::Both).expect("Shutdown failed");
    }
}

/*
TCP client 
sends input, and receives it back
upon disconnection: CTRL+C + Enter sends a shutdown, 
 therefore letting the server know of the disconnect
*/
