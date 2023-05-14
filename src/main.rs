use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream}, thread, time::Duration,
};

use server::ThreadPool;

const URL: &str = "127.0.0.1:7878";

fn main() {
    let listener = TcpListener::bind(URL).expect("Server should bind");
    let pool = ThreadPool::new(4);

    println!("Server listening on {}", URL);

    for stream in listener.incoming().take(2) {
        let stream = stream.expect("Stream should be ok");
        println!(
            "Connection established with {}",
            stream.peer_addr().expect("Peer should have address")
        );

        pool.execute(||handle_connection(stream));
    }

    println!("Server exiting");
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let response = match request_line.as_str() {
        "GET /hello HTTP/1.1" => {
            thread::sleep(Duration::from_secs(1));
            create_response_from_file(200, "OK", "hello.html")
        },
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(1));
            create_response_from_file(200, "OK", "hello.html")
        }
        _ => create_response_from_file(404, "Not Found", "404.html")
    };

    stream.write_all(response.as_bytes()).unwrap();
}

fn create_response_from_file(status_code: u32, status_text: &str, file_name: &str) -> String {
    let content = fs::read_to_string(file_name).unwrap();
    let length = content.len();
    format!("HTTP/1.1 {status_code} {status_text}\r\nContent-Length: {length}\r\n\r\n{content}")

}
