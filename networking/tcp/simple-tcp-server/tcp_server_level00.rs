use std::net::{ TcpListener };


fn main() {
    let listener = TcpListener::bind("127.0.0.1:8888")
                                .expect("not bound");
    println!("127.0.0.1:8888 running");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("new client: {}", stream.peer_addr().expect("wheee, fail"));
            }
            Err(e) => { println!("something is bad: {}", e); }
        }
    }
}

/* TCP server
diplays peer_addr() on connection
*/
