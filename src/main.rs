use tiny_http::{Server, Response};

fn main() {
    // Set the address and port for the server
    let server = Server::http("0.0.0.0:8000").unwrap();

    println!("Server running on http://0.0.0.0:8000");

    // Serve requests in a loop
    for request in server.incoming_requests() {
        let response = Response::from_string("
            <!DOCTYPE html>
            <html lang=\"en\">
            <head>
                <meta charset=\"UTF-8\">
                <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">
                <title>Simple Rust Webpage</title>
            </head>
            <body>
                <h1>Hello from Rust!</h1>
                <p>This is a basic web page served using Rust.</p>
            </body>
            </html>
        ");

        // Send the response to the client
        let _ = request.respond(response);
    }
}

