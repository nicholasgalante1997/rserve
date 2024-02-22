// We bring std::io::prelude and std::io::BufReader into scope
// to get access to traits and types that let us read from and write to the stream.
use std::fs;
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};

// What do we want to do in the simplest way
// We want to write a server that is capable
// of receiving a request for a public file
// and returning the contents of that file
// i.e. a static web server.

fn main() {
    // Create an instance of a TcpListener and bind it to port 7878
    // bind returns a Result<T, E> so we must unwrap() it to get the resultant T
    // TODO: explicitly handle a binding panic when we port this to modules
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // In the for loop in the main function,
    // instead of printing a message that says we made a connection,
    // we now call the new handle_connection function and pass the stream to it.
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    // we create a new BufReader instance that wraps a mutable reference to the stream.
    // BufReader adds buffering by managing calls to the std::io::Read trait methods for us.
    let buf_reader = BufReader::new(&mut stream);
    // We create a variable named http_request to collect the lines of the request.
    // We indicate that we want to collect these lines in a vector
    // by adding the Vec<_> type annotation.
    let http_request: Vec<_> = buf_reader
        .lines() // BufReader implements the std::io::BufRead trait, which provides the lines method. The lines method returns an iterator of Result<String, std::io::Error> by splitting the stream of data whenever it sees a newline byte.
        .map(|result| result.unwrap()) // To get each String, we map and unwrap each Result. The Result might be an error if the data isn’t valid UTF-8 or if there was a problem reading from the stream.
        .take_while(|line| !line.is_empty()) // The browser signals the end of an HTTP request by sending two newline characters in a row, so to get one request from the stream, we take lines until we get a line that is the empty string.
        .collect();

    // Once we’ve collected the lines into the vector,
    // we’re printing them out using pretty debug formatting
    // so we can take a look at the instructions the web browser is sending to our server.
    println!("Request: {:#?}", http_request);

    // This is the first line in our response.
    // It contains the protocol, status code, and status text
    let status_line = "HTTP/1.1 200 OK";
    // Request our index file, which exists at ./index.html
    // read_to_string returns a Result<String> so we need to unwrap it
    // in order to obtain its value (the String)
    let contents = fs::read_to_string("index.html").unwrap();
    // We'll use the length of the content in headers below
    let length = contents.len();
    // response follows the format
    // status_line CRLF
    // Headers CRLF
    // CRLF
    // Body
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    // we call as_bytes on our response to convert the string data to bytes.
    // The write_all method on stream takes a &[u8]
    // and sends those bytes directly down the connection.
    // Because the write_all operation could fail,
    // we use unwrap on any error result as before.
    stream.write_all(response.as_bytes()).unwrap();
}
