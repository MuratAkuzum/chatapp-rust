use std::{io::{BufReader, BufRead, Write}, net::{TcpListener, TcpStream}, thread};

const SOCKET: &str = "127.0.0.1:8080";

fn main() {
    // Initializing a socket/address for the server to: 
    let listener = TcpListener::bind(SOCKET).expect("!! Could not bind socket!");
    println!(">> Server has been started on {}", SOCKET);

    // Listening for and accept client connections: 
    for client_connection in listener.incoming() {
        match client_connection {
            // If connection is successful, print the info and create a thread for the connection: 
            Ok(client_connection) => {
                println!(">> Connection has been accepted from {:?}", client_connection.peer_addr().unwrap());
                thread::spawn(move || {handle_client(client_connection)});
            }
            // If connection fails, print the info: 
            Err(err) => {println!("Connection has failed due to error: {}", err)}
        }
    }
}

// Function to handle each connected client: 
fn handle_client(client_connection: TcpStream) {
    // Creating a buffer to hold data received from the client: 
    let mut data_receive = BufReader::new(client_connection);

    // Accepting the first data received as username for the client: 
    let mut username_buffer = String::new();
    data_receive.read_line(&mut username_buffer).unwrap();
    let username = username_buffer.trim();

    // Creating a loop to keep listening for data sent by the client: 
    loop {
        // Buffer to hold the converted data into string: 
        let mut data_buffer = String::new();

        // Breaking out of loop if the client stops sending data: 
        if data_receive.read_line(&mut data_buffer).is_err() {
            println!(">> {} has just left the server!", username);
            break;
        }

        // Writing the data received from client into string buffer and printing it: 
        data_receive.get_ref().write(data_buffer.as_bytes()).unwrap();
        println!(">> {}: {}", username, data_buffer.trim());
    }
}
