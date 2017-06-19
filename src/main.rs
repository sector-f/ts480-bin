extern crate ts480;
use ts480::*;

use std::env::args_os;
use std::process::exit;

fn main() {
    let args = args_os().collect::<Vec<_>>();
    if args.len() != 3 {
        println!("Usage: ts480-tx SERIAL_PORT DATA");
        exit(1);
    }

    let port = args.get(1).unwrap();
    let data = args.get(2).unwrap();

    let mut radio = match TS480::new(port) {
        Ok(radio) => radio,
        Err(e) => {
            println!("Could not connect to radio: {}", e);
            exit(1);
        },
    };

    if let Err(e) = radio.transmit(data.to_string_lossy().as_ref()) {
        println!("Error sending data to radio: {}", e);
        drop(radio);
        exit(1);
    }

    match radio.receive() {
        Ok(data) => {
            println!("{}", data);
        },
        Err(e) => {
            println!("Error receiving data from radio: {}", e);
            drop(radio);
            exit(1);
        },
    }
}
