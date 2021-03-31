use std::net::{ TcpStream, Shutdown };

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8888")
        .expect("Error connecting to the server");
    println!("Connected to server");
    stream.shutdown(Shutdown::Both).expect("Shutdown failed");
}

/*
TCP client 
simple connect and disconnect client
*/
