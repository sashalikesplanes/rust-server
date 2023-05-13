use std::{net::TcpListener, io::Write};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").expect("Server should bind");

    while let Some(stream) = listener.incoming().next() {
        let mut stream = stream.expect("Stream should be ok");
        println!("Connection established");
        println!("{}", stream.peer_addr().expect("Peer should have address"));

        let response = String::from("hello");
        stream.write(response.as_bytes()).expect("Stream write should succeed");
    }

    println!("Server exiting");
}
