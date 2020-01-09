# unix-stream: Rust UNIX socket demo
Bart Massey

This little Rust UNIX demo consists of two pieces. Say

     cargo run --bin pong

to start a UNIX domain socket listener bound to
`/tmp/unix-stream`. In a second terminal, do

    cargo run --bin ping

The `ping` client will send the line `"hello world"` to the
`pong` server once every quarter-second. The `pong` server
will take the received line and write it back to the client.
Both programs will print informational messages.

The `pong` server is not concurrent: only one `ping` client
can connect at a time.

-----

This work is licensed under the "MIT License". Please see
the file `LICENSE` in this distribution for license terms.
