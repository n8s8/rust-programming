extern crate term;
use std::net::{ TcpListener, TcpStream, Shutdown };

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8888")
                                .expect("not bound");
    let mut t = term::stdout().unwrap();
    println!("127.0.0.1:8888 running");
    for stream in listener.incoming() {
        match stream {
            Err(e) => { eprintln!("failed: {}", e); }
            Ok(stream) => {
                t.fg(term::color::BRIGHT_GREEN).unwrap();
                write!(t, "new client: ");
                t.reset().unwrap();
                println!("{}", stream.peer_addr().expect("wheee, fail"));
                t.fg(term::color::BRIGHT_RED).unwrap();
                write!(t, "client disconnected: ", );
                t.reset().unwrap();
                println!("{}", stream.peer_addr().expect("wheee, fail"));
                stream.shutdown(Shutdown::Both).expect("shutdown failed");
            }
        }
    }
}

/* TCP server 
accepts connection diplays peer_addr() and shutdown connection with him
*/
