extern crate rstatic;
extern crate clap;

use std::str::FromStr;
use clap::{Arg, App};

fn main() {
    let matches = App::new("rstatic")
        .version("0.1.3")
        .about("Static file serve by Rust")
        .author("Qing Yan <admin@zensh.com>")
        .arg(Arg::with_name("PORT")
            .short("p")
            .long("port")
            .help("Sets the port to listen, default to 3000")
            .required(false)
            .takes_value(true))
        .arg(Arg::with_name("PATH")
            .help("Sets the path to serve, default to current path")
            .required(false)
            .index(1))
        .get_matches();

    let mut port: u16 = 3000;
    let mut path = ".";

    if let Some(_port) = matches.value_of("PORT") {
        port = u16::from_str(_port).expect("Failed to read port");
    }

    if let Some(_path) = matches.value_of("PATH") {
        path = _path;
    }

    let serve = rstatic::Serve {port: port, path: path, maxage: 0};

    println!("rserve listen at: {}, serve for: {}", serve.port, serve.path);
    serve.listen();
}
