//! Test program for sockets
//! Note: this script is a quick-and-dirty script to demonstrate how TcpStream works only,
//! and is not an example of good Rust code. The error handling is of poor-quality.

use std::{
    io::{Read, Write},
    net::{Shutdown, TcpListener, TcpStream}, str::FromStr, time::Duration,
};

/// The socket address that will be used to make the connection.
const ADDR: &str = "127.0.0.1:60000";

/// The message that is shown at start
const HELP_MSG: &str = r#"
Command format:
    num instruction

    num: the number associated with the TcpStream (0 or 1)).
    instruction: the thing that you want the TcpStream to do.
        shutr: shutdown the read end of this TcpStream.
        shutw; shutdown the write end of this TcpStream.
        shutb: shutdown both ends of this TcpStream.
        close: drop the TcpStream, closing it.
        send: send the bytes 'abcd'.
        recv: try to recieve 4 bytes.
        quit: exit the program.
===============================================================
"#;

#[derive(Clone, Copy, Debug)]
enum Instruction {
    Shut(Shutdown),
    Close,
    Send,
    Recv,
    Quit,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Instruction::*;
        let symb = match s {
            "shutr" => Shut(Shutdown::Read),
            "shutw" => Shut(Shutdown::Write),
            "shutb" => Shut(Shutdown::Both),
            "close" => Close,
            "send" => Send,
            "recv" => Recv,
            "quit" => Quit,
            _ => return Err(())
        };
        Ok(symb)
    }
}

#[derive(Clone, Copy, Debug)]
struct Command {
    /// False if 0, True if 1
    pub num: bool,
    pub instruction: Instruction
}

/// Returns Some if a command could be made successfully, None otherwise.
fn make_command(num_str: &str, instruction_str: &str) -> Option<Command> {
    let num = match num_str {
        "0" => false,
        "1" => true,
        _ => return None,
    };

    let instruction = instruction_str.parse().ok()?;

    Some(Command {
        num,
        instruction,
    })
}

/// Reads a command from stdin. Repeatedly asks user for command until one is given.
fn read_command() -> Command {
    loop {
        print!(">>> ");
        let _ = std::io::stdout().flush();
        let mut line = String::new();
        std::io::stdin()
            .read_line(&mut line)
            .expect("reading from stdin should not fail");
        
        // parse command using match abuse
        let result: Option<Command> = match Vec::from_iter(line.trim().split_whitespace()).as_slice() {
            [num, instruction] => make_command(num, instruction),
            _ => None,
        };

        match result {
            Some(command) => return command,
            None => {
                println!("Error parsing command! Try again.");
                continue;
            }
        }
    }
}

/// Creates 2 connected TCP endpoints using 127.0.0.1:60000.
fn create_connection() -> [TcpStream; 2] {
    let listener = TcpListener::bind(ADDR).expect("binding should succeed");

    let stream0 = TcpStream::connect(ADDR).expect("connection should succeed");

    let (stream1, _addr) = listener
        .accept()
        .expect("there should be a connection to accept");

    let result = [stream0, stream1];
    for stream in &result {
        // Needed so reading doesn't take forever
        stream.set_read_timeout(Some(Duration::from_millis(100))).expect("Setting read timeout should not fail");
    }
    result
}

fn main() {
    use Instruction::*;
    let streams = create_connection();
    // Turn them into Somes (they become None when closed)
    let mut streams: [Option<TcpStream>; 2] = streams.map(|stream| Some(stream));
    
    println!("Connection established between endpoints through {ADDR}!");

    println!("{}", HELP_MSG);
    
    loop {
        let c = read_command();
        let index = c.num as usize;
        // Get stream, or read command again if it was closed
        let stream: &mut TcpStream = match &mut streams[index] {
            Some(stream) => stream,
            None => {
                println!("Error: stream {index} was closed!");
                continue;
            }
        };

        match c.instruction {
            Shut(how) => println!("Shutdown result: {:?}", stream.shutdown(how)),
            Close => {
                // drop that stream
                streams[index].take();
                println!("Closed stream {index}")
            },
            Send => println!("Send result: {:?}", stream.write(b"abcd")),
            Recv => println!("Receive result: {:?}", stream.read(&mut [0; 4])),
            Quit => { println!("Goodbye!"); return; },
        }
    }
}
