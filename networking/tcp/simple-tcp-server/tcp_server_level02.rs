extern crate term;
use std::net::{ TcpListener, Shutdown };
use std::io::{ Read, Write };


fn main() {
    let listener = TcpListener::bind("127.0.0.1:8888")
                                .expect("not bound");
    let mut t = term::stdout().unwrap();
    println!("127.0.0.1:8888 running");
    for stream in listener.incoming() {
        match stream {
            Err(e) => { eprintln!("failed: {}", e); }
            Ok(mut stream) => {
                t.fg(term::color::BRIGHT_GREEN).unwrap();
                write!(t, "new client: ");
                t.reset().unwrap();
                println!("{}", stream.peer_addr().expect("wheee, fail"));
                loop { 
                    let mut buffer = [0; 512];
                    if let Err(_) = stream.read(&mut buffer) { 
                        break;
                    };
                    if buffer == [0; 512] {
                        t.fg(term::color::BRIGHT_RED).unwrap();
                        write!(t, "client disconnected: ", );
                        t.reset().unwrap();
                        println!("{}", stream.peer_addr().expect("wheee, fail"));
                        stream.shutdown(Shutdown::Both).expect("shutdown failed");
                        break;
                    }
                    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
                    if let Err(_) = stream.write(&buffer[..]) { 
                        break;
                    };
                    if let Err(_) = stream.flush() { 
                        break;
                    };
                } 
            }
        }
    }
}


/* TCP server
accepts connection, displays peer_addr(), echoes incoming data
when peer terminates the connection, it displays that peer disconnected
*/
// https://riptutorial.com/rust/example/4404/a-simple-tcp-client-and-server-application--echo
// https://www.reddit.com/r/rust/comments/6tknm5/when_does_a_broken_pipe_occurs_in_a_tcp_stream/
