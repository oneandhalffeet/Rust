fn main() {
    // let string = String::from("😀😃😄😁😆😅");
    // let string_port = &string[12..];
    // let string_borrow: &str = &string;
    // let string_literal = "1234";

    // dbg!(&string);
    // dbg!(&string_port);
    // dbg!(&string_borrow);
    // dbg!(&string_literal);

    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}

struct Server{
    addr: String,

}


// Adding implementation for Server struct
impl Server {
    fn new(addr: String) -> Self{
        Self {
            addr
        }
    }

    fn run(self){
        println!("Listening on {}", self.addr);
    }
}