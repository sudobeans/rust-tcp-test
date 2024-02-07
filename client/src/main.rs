use std::{
    io::Read,
    net::{Shutdown, TcpStream},
};

fn line() {
    let mut string = String::new();
    std::io::stdin()
        .read_line(&mut string)
        .expect("reading from stdin should not fail");
}
fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:60000").expect("connecting should succeed");
    println!("Client connected!");

    println!("Hit enter to shutdown read end");
    line();
    stream
        .shutdown(Shutdown::Read)
        .expect("how could shutdown possibly fail");

    loop {
        println!("Hit enter to attempt to read");
        line();
        let mut buf = [0u8; 4];
        let result = stream.read(&mut buf);
        println!("reading result: {result:?}");
    }
}
