use std::io::prelude::*;
use std::net::{TcpStream};
use ssh2::Session;
use std::path::Path;

fn main() {
    // Connect to the local SSH server
    let tcp = TcpStream::connect("[IP:SSH_PORT]").unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    
    // Authenticate using keys
    let username = "[REMOTE_USERNAME]";
    let pubkey = Some(Path::new("/path/to/.ssh/key.pub"));
    let privatekey = Path::new("/path/to/.ssh/key");
    let passphrase = Some("[PASSPHRASE_TO_SSH_KEY]");
    sess.userauth_pubkey_file(username, pubkey, privatekey, passphrase).unwrap();
    assert!(sess.authenticated());

    // Executes ls command on remote machine, returns output, exits
    let mut channel = sess.channel_session().unwrap();
    channel.exec("ls").unwrap();
    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();
    println!("{}", s);
    channel.close().unwrap();
    println!("exit status: {}", channel.exit_status().unwrap());
}
