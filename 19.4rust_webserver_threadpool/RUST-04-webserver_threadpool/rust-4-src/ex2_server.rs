use std::thread;        // 线程
use std::net::{TcpListener, TcpStream, Shutdown}; // 网络
use std::io::{Read,Write};// io读写

fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 50]; // 50字节的buffer，初始化为0
    while match stream.read(&mut data) {
        Ok(size) => {
            // 回写数据
            stream.write(&data[0..size]).unwrap();
            true  //返回的是 while循环需要处理的值
        },
        Err(e)=> {
            println!("An error:{} occurred, terminating connection with {}", e, stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false  // 数据读取完毕
        }
    }{} 
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    println!("Server listening on port 3333");
    // accept connections and process them, spawning a new thread for each one
    for stream in listener.incoming() {
        match stream {
            Ok(stream)=> {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move||{
                    //连接成功
                    handle_client(stream);
                });
            },
            Err(e)=> {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }
    // 关闭服务
    drop(listener);
}