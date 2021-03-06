extern crate nickel;

use std::io::net::ip::Ipv4Addr;
use nickel::{ Nickel, Request, Response };

fn main() {
    let mut server = Nickel::new();
    
    fn a_handler (_request: &Request, response: &mut Response) { 
            response.send("What the...a Rust-powered web app?"); 
        }

    server.get("/", a_handler);
    server.listen(Ipv4Addr(0, 0, 0, 0), 8080);
}
