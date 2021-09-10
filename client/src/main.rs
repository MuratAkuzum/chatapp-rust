use std::{io::{stdin, stdout, BufReader, Write}, net::{TcpStream}};

const HOST: &str = "127.0.0.1:8080";

fn main() {
    // Connecting to the server, print the info, sent username as first message to the server: 
    let connection = TcpStream::connect(HOST).expect("!! Could not connect to the server!");
    println!(">> Connected to the server on {} with {}", HOST, connection.local_addr().unwrap());
    print!("Please enter a username: ");

    // Creating a buffer to hold messages to be sent to the server in bytes: 
    let data_send = BufReader::new(connection);
    // Looping for message input from client: 
    loop {
        // Writing some chars to look fancy before accepting inputs from user: 
        print!(">> ");
        stdout().flush().unwrap(); // Need to flush for print! macro to work
        
        // Creating buffer for strings to write into: 
        let mut input = String::new();

        // Reading the inputs and writing them into string buffer: 
        stdin().read_line(&mut input).expect("Please enter your input!");
        data_send.get_ref().write(input.as_bytes()).unwrap();
    }
}

