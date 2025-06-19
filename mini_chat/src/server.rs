use std::net::{TcpListener, TcpStream};
use std::io::{BufReader, BufRead, Write};
use std::sync::{Arc, Mutex};
use std::thread;

pub fn run() {
    let listener = TcpListener::bind("127.0.0.1:9000").unwrap();
    let clients = Arc::new(Mutex::new(Vec::new()));
    println!("Chat server running on 127.0.0.1:9000");
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let clients = Arc::clone(&clients);
        thread::spawn(move || {
            let mut writer = stream.try_clone().unwrap();
            clients.lock().unwrap().push(writer.try_clone().unwrap());
            let reader = BufReader::new(stream);
            for line in reader.lines() {
                let msg = line.unwrap();
                let mut clients = clients.lock().unwrap();
                for client in clients.iter_mut() {
                    let _ = writeln!(client, "{}", msg);
                }
            }
        });
    }
}