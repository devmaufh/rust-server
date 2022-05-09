fn main() {
    let server = Server::new("127.0.0.1".to_string(), "8080".to_string());
    server.run();
}

struct Server {
    url: String,
    port: String,
}

impl Server {
    fn new(url: String, port: String) -> Self {
        Self {
            url,
            port,
        }
    }

    fn run(self) {
        println!("Rust 🦀 server listening on {}:{}", self.url, self.port);
    }
}
