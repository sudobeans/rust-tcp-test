This is a demonstration of what I believe to be an error in the Rust documentation.

The description of [`std::net::Shutdown`](https://doc.rust-lang.org/std/net/enum.Shutdown.html) says this:

> The reading portion of the [`TcpStream`](https://doc.rust-lang.org/std/net/struct.TcpStream.html) should be shut down.

> All currently blocked and future [reads](https://doc.rust-lang.org/std/io/trait.Read.html) will return `Ok(0)`.

I found from my testing that calling `.shutdown(Shutdown::Read)` does not actually guarantee that future `read`s will return `Ok(0)`. 

After calling `shutdown`, I found that calling `TcpStream::read` always returns immediately. However, if a message has been sent to the TcpStream, it will instead return `Ok(n)`, where n is the number of bytes read.

Proof:

1. `cd` into the `server` folder and type `cargo run` to start the server.

2. Open a new terminal, `cd` into the `client` folder, and type `cargo run` to start the client.

3. Shutdown the read end of the client program.

4. Hit enter in the client. It says the result of reading is `Ok(0)`.

5. Hit enter in the server to send a message.

6. Hit enter in the client. It says the result of reading is `Ok(4)` despite the description of `std::net::Shutdown`.