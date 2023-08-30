pub struct Server{
    addr: String,

}


// Adding implementation for Server struct
impl Server {
    pub fn new(addr: String) -> Self{
        Self {
            addr
        }
    }

    pub fn run(self){
        println!("Listening on {}", self.addr);
    }
}