use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::str::from_utf8;
use std::thread;
use std::time;

fn simple_server() {
    let listener = TcpListener::bind("localhost:6379").unwrap();
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        let ping = b"PING\r\n";
        let pong = b"+PONG\r\n";

        let mut data = [0 as u8; 10]; // using 50 byte buffer
        let size = stream.read(&mut data).unwrap();
        assert_eq!(size, 6);
        assert_eq!(data[..size], ping[..]);

        let size = stream.write(pong).unwrap();
        assert_eq!(size, 7);
        break; // exit server after first request
    }
    drop(listener);
}

fn simple_client() {
    let mut stream = TcpStream::connect("localhost:6379").unwrap();

    let ping = b"PING\r\n";
    let size = stream.write(ping).unwrap();
    assert_eq!(size, 6);

    let mut data = [0 as u8; 10];
    let size = stream.read(&mut data).unwrap();
    let text = from_utf8(&data[..size]).unwrap();
    println!("Resp: {:?} => {}", &data[..size], text);
    assert_eq!(text, "+PONG\r\n");
}

fn main() {
    // start a mock server
    // comment it if test redis server
    thread::spawn(move || {
        simple_server();
    });
    thread::sleep(time::Duration::from_millis(100));
    simple_client();
}
