use std::{io::Write, net::TcpListener};

fn line() {
    let mut string = String::new();
    std::io::stdin()
        .read_line(&mut string)
        .expect("reading from stdin should not fail");
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:60000").expect("binding should succeed");
    println!("Server listening!");

    let (mut stream, addr) = listener
        .accept()
        .expect("there should be a connection to accept");
    println!("Server connected to {:?}", addr);

    loop {
        println!("Hit enter to send message to client");
        line();
        let msg = b"crab";
        let result = stream.write(msg);

        println!("Sent message and got result {:?}", result);
    }
}
