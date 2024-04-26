// Namespaces
use std::net::UdpSocket;

// Rough
struct Server<'a> {
    socket: UdpSocket,
    port: &'a str,
    hostname: &'a str,
}

impl Server<'_> {
    fn new<'a>(hostname: &'a str, port: &'a str) -> Server<'a> {
        return Server {
            socket: match UdpSocket::bind(format!("{}:{}", &hostname, &port)) {
                Ok(socket) => socket,
                Err(error_message) => panic!("{}", error_message)
            },
            port: &port,
            hostname: &hostname,
        };
    }

    fn get_socket(&self) -> &UdpSocket {
        return &self.socket;
    }
}

struct Yep<'a, 'b> {
    x: &'a i32,
    y: &'b i32,
}

fn test<'a, 'b>(x: &'a i32, y: &'b i32) -> &'a i32 {
    if *x > *y {
        return x;
    }
    return &0;
}

fn main() {
    let x: i32 = 10;
    let v: &i32;
    {
        let y: i32 = 2;
        let f: Yep<'_, '_> = Yep {
            x: &x,
            y: &y,
        };
        v = test(&f.x, &f.y);
    }
    println!("{}", v);

    // Create a socket
    let my_socket: Server = Server::new("127.0.0.1", "3400");

    // Input loop
    loop {
        // Create a new input buffer of unsigned bits
        let mut input_buffer: [u8; 128] = [1; 128];

        // Retrieve data
        match my_socket.get_socket().recv(&mut input_buffer) {
            Ok(received_data) => println!("Data received! Size: {}", received_data),
            Err(error_message) => println!("Skip! {}", error_message)
        };

        // Debug
        println!("As a string       => {}", String::from_utf8_lossy(&input_buffer));
        println!("Raw bytes         => {:#?}", &input_buffer[0..8])
    }

}
