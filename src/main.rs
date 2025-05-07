use std::{env, io::{self, Write}, net::{IpAddr, TcpStream}, process, sync::mpsc::{channel, Sender}, thread};

use ip_sniffer::Arguments;

const MAX_PORT: u16 = 65535;

fn scan(tx: Sender<u16>, start_port: u16, ip_addr: IpAddr, num_threads: u16) {
    let mut port = start_port + 1;

    loop {
        match TcpStream::connect((ip_addr, port)) {
            Ok(_) => {
                print!(".");
                io::stdout().flush().unwrap();
                tx.send(port).unwrap();
            }
            Err(_) => {}
        }

        if (MAX_PORT - port) <= num_threads {
            break;
        }
        port += num_threads;
    }
}


fn main() {
    let arguments = Arguments::build(env::args()).unwrap_or_else(
        |err| {
            if err.contains("help") {
                process::exit(0);
            } else {
                eprintln!("Error: {err}");
                process::exit(0);
            }
        }
    );

    let num_threads = arguments.threads;
    let ip_addr = arguments.ip_addr;
    let (tx, rx) = channel();
    for i in 0..num_threads {
        let tx = tx.clone();

        thread::spawn(move || {
            scan(tx, i, ip_addr, num_threads);
        });
    }

    let mut out = vec![];
    drop(tx);
    for p in rx {
        out.push(p);
    }

    println!("");
    out.sort();
    for v in out {
        println!("Port {} is open", v);
    }
}
