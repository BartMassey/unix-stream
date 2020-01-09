use std::io::prelude::*;
use std::os::unix::net::UnixStream;

fn main() {
    // Time interval for sleeps.
    let interval = std::time::Duration::from_millis(250);

    // Message to ping with.
    let message = "hello world\n";

    // Connect to the server socket and set up a line reader
    // to receive messages.
    let mut stream = UnixStream::connect("/tmp/unix-stream").unwrap();
    let reader = stream.try_clone().unwrap();
    let mut reader = std::io::BufReader::new(reader);
    eprintln!("ping: starting");

    // Loop forever sending a ping and listening for the
    // response every interval.
    loop {
        write!(stream, "{}", message).unwrap();
        stream.flush().unwrap();
        let mut response = String::new();
        reader.read_line(&mut response).unwrap();
        assert_eq!(response, message);
        eprint!("ping: response: {}", response);
        std::thread::sleep(interval);
    }
}
