use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::{io, thread};

fn handle_client(mut stream: TcpStream) {
    let client_addr = stream.peer_addr().expect("Error reading client address!");
    //declaring an array named buffer with 512 bytes and assigning value 0 to all positions.
    let mut buffer = [0; 512];

    loop {
        //Taking the number of bytes red to the variable bytes_read.
        let bytes_read = match stream.read(&mut buffer) {
            Ok(0) => {
                println!("{} disconnected", client_addr);
                break;
            }
            Ok(n) => n,
            Err(e) => {
                println!("Error {} reading from {}", e, client_addr);
                break;
            }
        };

        //Attenpting to write to the buffer arraycreated before.
        match stream.write(&buffer[..bytes_read]) {
            Ok(_) => {}
            Err(e) => {
                println!("Error {} writing to client {}", e, client_addr);
                break;
            }
        };

        if let Err(e) = stream.flush() {
            eprintln!("Error {} flushing to a{}", e, client_addr);
            break;
        }

        //Loop ends when the handler deals with a client and the message gets printed.
        println!(
            "Connection handler done dealing with client {}",
            client_addr
        );
    }
}

fn main() -> io::Result<()> {
    //binded a port and claimed ip address for local hosting.
    let listner = TcpListener::bind("127.0.0.1:7878")?;
    println!("Echo server listning on 127.0.0.1:7878");

    //itereating through the listner data using stream and checking match for stream.
    for stream in listner.incoming() {
        match stream {
            Ok(strm) => {
                thread::spawn(move || {
                    handle_client(strm);
                });
            }
            Err(e) => {
                eprintln!("Error {} accepting connection", e);
            }
        }
    }

    Ok(())
}
