# tiny-server
A barebones implementation for a simple http-server inspired by axum like API

# TODO
- [X] Set up a socket to listen on a specific port.
- [X] Accept incoming connections in a loop.
- [X] Read the raw HTTP request from the client.
- [X] Parse the HTTP request to extract the method, path, and protocol.
- [X] Implement handling for GET requests.
- [X] Return a basic HTTP response with a status line, headers, and body.
- [X] Add support for query parameters by parsing the request URL.
- [ ] Add support for serving static files (e.g., HTML, CSS, JS).
- [ ] Implement handling for 404 Not Found responses.
- [ ] Handle POST requests and read the request body.
- [ ] Implement response headers for Content-Type and Content-Length.
- [ ] Add support for custom error pages.
- [ ] Implement routing to handle different paths dynamically.
- [ ] Add a basic logging mechanism to log requests and responses.
- [ ] Serve files with MIME type detection based on the file extension.
- [ ] Add support for HTTP/1.1 keep-alive connections.
- [ ] Implement support for CORS headers.
- [ ] Add support for URL-encoded form submissions.
- [ ] Add support for serving JSON responses.
- [ ] Implement a mechanism to handle request timeouts.
- [ ] Add basic support for HTTP status codes (e.g., 200, 404, 500).
- [ ] Add a configuration file or environment variables for server settings (e.g., port, root directory).


# References
- https://datatracker.ietf.org/doc/html/rfc2616
- https://fasterthanli.me/articles/the-http-crash-course-nobody-asked-for
