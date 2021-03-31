extern crate term;

use std::net::{ TcpListener, TcpStream, Shutdown };
use std::thread;
use std::io::{ Read, Write, Error };

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8888")
        .expect("not bound");

    let mut t = term::stdout().unwrap();
    t.fg(term::color::BRIGHT_GREEN).unwrap();
    println!("{:?}", listener.local_addr().unwrap());
    t.reset().unwrap();

    for stream in listener.incoming() {
        match stream {
            Err(e) => eprintln!("error: {}", e),
            Ok(stream) => {
                thread::spawn(move || {
                    handle(stream).unwrap_or_else(|error|
                    eprintln!("{:?}", error));
                });
            }
        }
    }

}

fn handle(mut stream: TcpStream) -> Result<(), Error> {
    let mut t = term::stdout().unwrap();

    t.fg(term::color::BRIGHT_GREEN).unwrap();
    write!(t, "new client: ");
    t.reset().unwrap();
    println!("{}", stream.peer_addr().expect("wheee, fail"));

    loop {
        let mut buffer = [0; 512];

        if let Err(_) = stream.read(&mut buffer) {
            return Ok(())
        };

        if buffer == [0; 512] {
            t.fg(term::color::BRIGHT_RED).unwrap();
            write!(t, "client disconnected: ", );
            t.reset().unwrap();
            println!("{}", stream.peer_addr().expect("wheee, fail"));
            stream.shutdown(Shutdown::Both).expect("shutdown failed");
            return Ok(())
        }

        println!("Data: {}", String::from_utf8_lossy(&buffer[..]));
        
        if let Err(_) = stream.write(&buffer[..]) { 
            return Ok(())
        };

        if let Err(_) = stream.flush() { 
            return Ok(())
        };
    }
        
}

/*
TCP server
implementing threading, to handle more connections simultaneously
also it is organised in a function instead of just being thrown straight in the main :-)


https://doc.rust-lang.org/nightly/std/thread/
*/
