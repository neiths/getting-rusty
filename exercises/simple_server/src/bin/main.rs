use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs;
use simple_server::ThreadPool;
use std::thread;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    if buffer.starts_with(get) {
        serve_html(&mut stream, "index.html");
    } else if buffer.starts_with(sleep) {
        thread::sleep(std::time::Duration::from_secs(5));
        serve_html(&mut stream, "index.html");
    } else {
        serve_html(&mut stream, "404.html");
    }
}

fn serve_html(stream: &mut TcpStream, filename: &str) {
    let status_line = "HTTP/1.1 200 OK\r\n\r\n";
    let contents = fs::read_to_string(filename).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}