use std::io::{Read,Write,Result as Res};
use std::net::{TcpStream,TcpListener};
use std::thread;
use std::time::Duration;
use std::process::Command;

fn main() {
  let urls = [
    //keeping other programs alive
    "https://gloriousdevelopment.tk/",
    "https://Glorious-Mod-Current-Version.illuminateweb.repl.co"
  ];
  
  let _handle = thread::spawn(|| {
    let listener = TcpListener::bind("0.0.0.0:80").unwrap();
    
    for i in listener.incoming() {
      if let Ok(x) = i {
        handle(x);
      }
    }
  });
  
  loop {
    for url in &urls {
      get(url);
    }
    thread::sleep(Duration::from_secs(360));
  }
}

fn get(url: &str) {
  Command::new("curl")
    .arg(&format!("https://{}.repl.co",
      url))
    .spawn();
}

fn handle(mut stream: TcpStream) -> Res <()> {
  let mut r = [0;1024];
  stream.read(&mut r)?;
  stream.write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 5\r\n\r\nhello")?;
  stream.flush()?;
  Ok(())
}