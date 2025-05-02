use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::process;

fn main() {
    let mut stream = match TcpStream::connect("127.0.0.1:7878") {
        Ok(stream) => {
            println!("Succesfully connected to the server 127.0.0.1:7878");
            stream
        }
        Err(_e) => {
            eprintln!("Error connecting to server 127.0.0.1:7878");
            eprintln!("Make sure server is running!");
            process::exit(1);
        }
    };

    println!("Type a message and the server echos it back.");

    loop {
        let mut input_line = String::new();
        print!(">");
        io::stdout().flush().expect("Unable to flush error.");

        match io::stdin().read_line(&mut input_line) {
            Ok(0) => {
                println!("exiting client.");
                break;
            }
            Ok(_) => {
                let message = input_line.trim();
                if message.is_empty() {
                    continue;
                }
                match stream.write(message.as_bytes()) {
                    Ok(_) => {
                        let mut buffer = [0; 512];
                        match stream.read(&mut buffer) {
                            Ok(bytes_read) => {
                                let echoed_data = String::from_utf8_lossy(&buffer[..bytes_read]);
                                println!("Sercer echoed: {}", echoed_data);
                            }
                            Err(_e) => {
                                eprintln!("Error reading from stream!");
                                break;
                            }
                        };
                    }
                    Err(_e) => {
                        eprintln!("Error writing message to stream as bytes.");
                        break;
                    }
                };
            }
            Err(_e) => {
                eprintln!("Error reading from client!");
                break;
            }
        }
    }
}
