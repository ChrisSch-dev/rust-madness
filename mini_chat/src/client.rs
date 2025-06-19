use std::io::{self, Write, BufRead, BufReader};
use std::net::TcpStream;
use std::thread;

pub fn run() {
    let mut stream = TcpStream::connect("127.0.0.1:9000").unwrap();
    let mut stream2 = stream.try_clone().unwrap();

    // Reader thread
    thread::spawn(move || {
        let reader = BufReader::new(stream2);
        for line in reader.lines() {
            if let Ok(msg) = line {
                println!("{}", msg);
            }
        }
    });

    // Writer (main) thread
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let msg = line.unwrap();
        let _ = writeln!(stream, "{}", msg);
    }
}