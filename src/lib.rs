use std::{
    io::{self, BufRead, Read, Write},
    net::TcpStream,
};

#[macro_use]
extern crate log;
use bufstream::BufStream;
use chrono::{DateTime, Local};
use regex::Regex;

#[derive(Debug)]
struct Request {
    http_version: String,
    method: String,
    path: String,
    time: DateTime<Local>,
}

pub fn handle_client(stream: TcpStream) -> Result<(), failure::Error> {
    info!("handle_client");
    let mut buf = BufStream::new(stream);
    let mut request_line = String::new();

    buf.read_line(&mut request_line)?;

    match parse_request(request_line.trim()) {
        Ok(request) => {
            // ここでリクエストを処理する
            info!("{:#?}", request);
        }
        Err(e) => {
            error!("error: {}", e);
            error!("Bad request: {}", request_line);
        }
    }
    Ok(())
}

fn parse_request(req: &str) -> Result<Request, failure::Error> {
    // ^(GET|POST|PUT|DELETE|UPDATE) (/[a-zA-Z0-9\.-]*) HTTP/([0-9\.]*)$
    // このままだと..で親ディレクトリにアクセスできてしまう
    let re = Regex::new(r"^(GET|POST|PUT|DELETE|UPDATE) (/[a-zA-Z0-9\.-]*) HTTP/([0-9\.]*)$")?;
    match re.captures(req) {
        Some(ref cap) if cap.len() == 4 => Ok(Request {
            http_version: cap.get(3).unwrap().as_str().to_owned(),
            method: cap.get(1).unwrap().as_str().to_owned(),
            path: cap.get(2).unwrap().as_str().to_owned(),
            time: Local::now(),
        }),
        _ => {
            error!("{:#?}", req);
            error!("{:#?}", re.captures(req));
            return Err(failure::err_msg("cant match"));
        }
    }
}
