use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use my_server::ThreadPool;

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let request_line: Vec<_> = http_request
        .get(0)
        .unwrap()
        .split_ascii_whitespace()
        .collect();
    let path = request_line.get(1).unwrap().to_string();

    let (status_line, file_path) = match path.as_str() {
        "/" => ("HTTP/1.1 200 OK", "index.html"),
        "/sleep" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "index.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let res_body = fs::read_to_string(file_path).unwrap();
    let res_headers = format!("Content-Type: html;\r\nContent-Length:{}", res_body.len());
    let res = format!("{status_line}\r\n{res_headers}\r\n\r\n{res_body}");
    stream.write_all(&res.as_bytes()).unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}
