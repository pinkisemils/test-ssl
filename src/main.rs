extern crate openssl;

use openssl::ssl::{SslConnector, SslMethod};

fn main() {
    openssl::init();
    let mut ssl_builder = SslConnector::builder(SslMethod::tls()).expect("failed to construct a builder");
    ssl_builder.set_ca_file("ca.pub").expect("failed to set custom cert path");
    let ssl = ssl_builder.build();
    let e = ssl.configure();
    match e {
        Ok(_) => println!("good conf"),
        Err(_) => println!("bad conf"),
    }
    println!("Hello, world!");
}
