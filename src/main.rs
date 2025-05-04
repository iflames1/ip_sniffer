use std::{env, io::{self, Write}, net::{IpAddr, TcpStream}, process, str::FromStr, sync::mpsc::{channel, Sender}, thread};

const MAX_PORT: u16 = 65535;

struct Arguments {
    flag: String,
    ip_addr: IpAddr,
    threads: u16,
}

impl Arguments {
    fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments");
        } else if args.len() > 4 {
            return Err("Too many arguments");
        }

        let f = args[1].clone();

        if let Ok(ip_addr) = IpAddr::from_str(&f) {
            return Ok(Arguments {
                flag: String::from(""),
                ip_addr,
                threads: 4,
            })
        } else {
            let flag = args[1].clone();
            if flag.contains("-h") || flag.contains("-help") && args.len() == 2 {
                println!("Usage: -j to select how many threads you want \r\n -h or -help to show this help message");
                return Err("help");
            } else if flag.contains("-h") ||flag.contains("-help") {
                return Err("Too many arguments");
            } else if flag.contains("-j") {
                let ip_addr = match IpAddr::from_str(&args[3]) {
                    Ok(s) => s,
                    Err(_) => {
                        return Err("Not a valid IP address");
                    }
                };

                let threads = match args[2].parse::<u16>() {
                    Ok(s) => s,
                    Err(_) => {
                        return Err("Failed to parse thread number");
                    }
                };

                return Ok(Arguments{
                    threads,
                    flag,
                    ip_addr,
                });
            } else {
                return Err("Invalid syntax");
            }
        }
    }
}


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
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let arguments = Arguments::new(&args).unwrap_or_else(
        |err| {
            if err.contains("help") {
                process::exit(0);
            } else {
                eprintln!("{program} problem parsing arguments: {err}");
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
