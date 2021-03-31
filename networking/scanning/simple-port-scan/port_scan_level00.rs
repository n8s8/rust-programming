use std::net::{ TcpStream, Shutdown, SocketAddr };
use std::time::Duration;

fn main() {
    let mut open_ports = vec![];

    for i in 1..=65535 {
        let ip = "192.168.0.1:".to_string();
        let port = i.to_string();
        let addr = ip + &port;
        let socket: SocketAddr = addr.parse().unwrap();
        let stream = TcpStream::connect_timeout(&socket, Duration::from_millis(1500));
        match stream {
            Ok(x) => {
                println!(
                    "Open {}", &socket
                );

                if let Err(e) = x.shutdown(Shutdown::Both) {
                    println!("Shutdown stream error {}", &e);
                }
                open_ports.push(socket);
            }
            Err(e) => {
                let _error_string = e.to_string();
                // println!("{}", _error_string);
            }
        }
    }
    println!("\tOpen ports: ");
    for i in open_ports {
        println!("Open: {:?}", i);
    }
}

/*
Port Scanner
Works 100% locally, however,
on internet, or networks in general, it gets stuck on port
that is not open. This behaviour is only present when scanning a 
remote host.
*/