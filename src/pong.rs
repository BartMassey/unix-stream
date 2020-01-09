use std::io::{BufRead, Write};
use std::os::unix::net::UnixListener;

fn main() {
    // Clear out the old dead socket, if any.
    let path = "/tmp/unix-stream";
    std::fs::remove_file(path).unwrap_or_else(|e| match e.kind() {
        std::io::ErrorKind::NotFound => (),
        _ => panic!("{}", e),
    });

    // Create a new socket. Each time a client connects,
    // interact with it.
    let listener = UnixListener::bind(path).unwrap();
    for stream in listener.incoming() {
        // Create a line reader for this stream.
        let mut stream = stream.unwrap();
        let reader = stream.try_clone().unwrap();
        let reader = std::io::BufReader::new(reader);
        eprintln!("pong: stream");

        // Process lines from the client.
        for response in reader.lines() {
            let response = response.unwrap();
            eprintln!("pong: message: {}", response);
            writeln!(stream, "{}", response).unwrap();
            stream.flush().unwrap();
            eprintln!("pong: responded");
        }
    }
}
