use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::path::{Path, PathBuf};

pub fn run(dir: String, port: u16) {
    let listener = TcpListener::bind(("127.0.0.1", port)).unwrap();
    for stream in listener.incoming() {
        let dir = dir.clone();
        thread::spawn(move || {
            let stream = stream.unwrap();
            handle_connection(stream, &dir);
        });
    }
}

fn handle_connection(mut stream: TcpStream, dir: &str) {
    let mut buffer = [0; 1024];
    if stream.read(&mut buffer).is_err() {
        return;
    }
    let request = String::from_utf8_lossy(&buffer[..]);
    let path = parse_path(&request);
    let mut file_path = PathBuf::from(dir);
    file_path.push(&path[1..]); // remove leading '/'
    let response = if file_path.is_file() {
        let contents = fs::read(&file_path).unwrap_or_else(|_| b"Not found".to_vec());
        let mime = get_mime(file_path.extension().and_then(|s| s.to_str()).unwrap_or(""));
        format!(
            "HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n",
            mime,
            contents.len()
        ).into_bytes().into_iter().chain(contents).collect()
    } else if file_path.is_dir() {
        let listing = dir_list(&file_path);
        format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
            listing.len(),
            listing
        ).into_bytes()
    } else {
        b"HTTP/1.1 404 NOT FOUND\r\n\r\nNot found".to_vec()
    };
    let _ = stream.write_all(&response);
}

fn parse_path(request: &str) -> String {
    let mut lines = request.lines();
    if let Some(first_line) = lines.next() {
        let parts: Vec<&str> = first_line.split_whitespace().collect();
        if parts.len() > 1 {
            return parts[1].to_string();
        }
    }
    "/".to_string()
}

fn dir_list(dir: &Path) -> String {
    let mut html = format!("<h1>Index of {}</h1><ul>", dir.display());
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().into_owned();
            html.push_str(&format!("<li><a href=\"{}\">{}</a></li>", name, name));
        }
    }
    html.push_str("</ul>");
    html
}

fn get_mime(ext: &str) -> &'static str {
    match ext {
        "html" => "text/html",
        "css" => "text/css",
        "js" => "application/javascript",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "svg" => "image/svg+xml",
        "txt" => "text/plain",
        _ => "application/octet-stream",
    }
}