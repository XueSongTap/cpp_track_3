mod lib;
use lib::ThreadPool;
use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;




fn main() {
    println!("Hello, world!");
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
    let pool = ThreadPool::new(4);
    // for stream in listener.incoming().take(2) {
    //     let stream = stream.unwrap();
    //     pool.execute(|| {
    //         handle_connection(stream);
    //     });
    // }

    for stream in listener.incoming() {
        match stream {
            Ok(stream)=> {
                pool.execute(|| {
                    handle_connection(stream);
                });
            },
            Err(e)=> {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }
    println!("Shutting down.");
}


fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    println!("filename:{}", filename);

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}