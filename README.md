# My Rust Echo Protocol Project! 🎉

Hey there! Welcome to my echoServer project where I messed around with network stuff in Rust.

This project is a super simple implementation of the classic **Echo Protocol** (RFC 862). Basically, it's a server that just listens for messages and sends whatever it gets *right back* to the sender, and a client to talk to it.

I built this primarily to learn Rust and low-level network programming concepts like sockets, binding, listening, reading, and writing data over TCP streams. It was a task to figuring it out! Especially as a noob in Rust and software developing. Maybe its a skill issue, but anyway happy reading further!!!

## What's Inside?

* A basic **TCP Echo Server** that can handle multiple clients at once using threads.
* A simple **TCP Echo Client** that lets you type messages and see them echoed back.

## Getting Started

You'll need **Rust and Cargo** installed on your machine. If you don't have it yet, the easiest way is usually through `rustup`.

1.  **Clone this repository:**
    ```bash
    git clone <URL_of_your_github_repo>
    cd <your_repo_folder_name>
    ```
    (Replace `<URL_of_your_github_repo>` and `<your_repo_folder_name>` with the actual details!)

2.  **Build the project:**
    Since this is a Cargo workspace with two binaries (server and client), you can build both with one command from the root directory:
    ```bash
    cargo build --workspace
    ```
    This will create the executable files in the `target/debug/` directory.

## How to Run

You'll need two separate terminal windows for this!

1.  **Start the Server:**
    In the first terminal, run the server executable. It will bind to `127.0.0.1:7878` and start listening.
    ```bash
    cargo run --package server
    ```
    (Or run the executable directly: `./target/debug/server`)
    You should see output like `Echo server listening on 127.0.0.1:7878`. Keep this terminal open!

2.  **Start the Client:**
    In the second terminal, run the client executable. It will try to connect to the server you just started.
    ```bash
    cargo run --package client
    ```
    (Or run the executable directly: `./target/debug/client`)
    If the connection is successful, you'll see a `>` prompt.

3.  **Chat!**
    Now, type messages into the client terminal and press Enter. The client sends the message to the server, the server echoes it back, and the client prints the echoed response!

    To stop the client, you can usually press `Ctrl+D` (End of Input). To stop the server, press `Ctrl+C`.

## How it Works (Briefly)

* The **Server** uses `std::net::TcpListener` to `bind` to an address and port and `listen` for incoming connections.
* It uses `listener.incoming()` in a loop to `accept` new connections.
* For each new connection (`std::net::TcpStream`), it spawns a new `std::thread` using `thread::spawn` to handle that client's conversation independently.
* The `handle_client` function running in each thread uses `stream.read()` to get data from the client and `stream.write()` to send it back, using a fixed-size buffer and looping until the client disconnects or an error occurs.
* The **Client** uses `std::net::TcpStream::connect` to initiate a connection to the server's address and port.
* It then enters a loop, reading input from the user (`std::io::stdin().read_line()`), writing that input to the server's stream (`stream.write()`), and reading the echoed response back (`stream.read()`) before printing it.

This project helped me to get a hands on experience on rust and networking basics. And personally coming from a C/C++ world perspective, i understand Rust's approach with results. Even though those were confusing at first, writing things in rust really make you think in a rust way.

Hope this helps anyone checking out the repo! Feel free to poke around the code in the `server/src/main.rs` and `client/src/main.rs` files.
