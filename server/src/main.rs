use server::Server;
use http::Method;

mod server;
mod http;


fn main() {
    // let string = String::from("😀😃😄😁😆😅");
    // let string_port = &string[12..];
    // let string_borrow: &str = &string;
    // let string_literal = "1234";

    // dbg!(&string);
    // dbg!(&string_port);
    // dbg!(&string_borrow);
    // dbg!(&string_literal);

    // let get = Method::GET("abcd".to_string());
    // let post = Method::POST;
    // let delete = Method::DELETE(123);
    // let put = Method::PUT;

    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}




