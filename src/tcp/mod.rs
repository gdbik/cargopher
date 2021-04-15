use std::net::TcpStream;
use std::io::{Write, Read};

pub struct tcp {}

impl tcp {
   pub fn go_to(host: &str, port: u16, path: &str) -> Result<String, ()> {
      let url = format!("{}:{}", host, port);
      let stream = TcpStream::connect(&url);
      match stream {
         Ok(mut res) => {
            let selector = format!("{}\r\n", path);
            res.write(selector.as_bytes()).unwrap();
            res.flush().unwrap();

            let mut data: Vec<u8> = vec![];
            res.read_to_end(&mut data).unwrap();

            Ok(String::from_utf8_lossy(&data).to_string())
         },
         Err(e) => {
            println!("{}", e);
            Err(())
         }
      }
   }
}