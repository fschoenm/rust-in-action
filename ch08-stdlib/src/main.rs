use std::io::prelude::*;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let host = "www.rustinaction.com:80";

    let mut conn = TcpStream::connect(host)?;

    conn.write_all(b"GET / HTTP/1.1\r\n")?;
    conn.write_all(b"Host: www.rustinaction.com\r\n\r\n")?;

    std::io::copy(&mut conn, &mut std::io::stdout())?;

    Ok(())
}
