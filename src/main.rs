use std::fs;
use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use webserver::ThreadPool;
use std::time::Duration;
use std::thread;
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);
    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();
        pool.execute(|| handle_connection(stream));
        // println!("Connection established");
    }
    println!("Shutting down...");
}

fn handle_connection(mut stream:TcpStream) {
    let mut buffer  = [0; 512];
    stream.read(&mut buffer).unwrap();
    // 转为字节
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    let (status,filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n","hello.html")
    } else if buffer.starts_with(sleep){
        thread::sleep(Duration::from_secs(3));
        ("HTTP/1.1 200 OK\r\n\r\n","hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n","404.html")
    };
    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status, contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    // println!("Request : {}", String::from_utf8_lossy(&buffer[..]));
}
