use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs;
use std::thread;
use std::time::Duration;
use hello::ThreadPool;
fn main() {
	let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

	let pool = ThreadPool::new(4);

	for stream in listener.incoming() {
		let stream = stream.unwrap();

		pool.execute(||{run_server(stream)})
	}
}
fn run_server(mut stream: TcpStream){
	let mut buf = [0; 512];
	
	stream.read(&mut buf).unwrap();
	
	println!("Request: {}", String::from_utf8_lossy(&buf[..]));

	let index = b"GET / HTTP/1.1\r\n";
	let sleep = b"GET /sleep HTTP/1.1\r\n";

	let mut contents = fs::read_to_string("404.html").unwrap();
	let mut response = format!("HTTP/1.1 404 NOT FOUND\r\n\r\n{}", contents);

	if buf.starts_with(index){
		contents = fs::read_to_string("index.html").unwrap();
		response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);	
	}
	else if  buf.starts_with(sleep){
		contents = fs::read_to_string("index.html").unwrap();
		response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);
		thread::sleep(Duration::from_secs(10));
	}
	

	stream.write(response.as_bytes()).unwrap();
	stream.flush().unwrap();
}