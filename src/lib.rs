use std::{error::Error, io::{self, Write}, net::{IpAddr, TcpStream}, str::FromStr, sync::mpsc::{channel, Sender}, thread};

const MAX_PORT: u16 = 65535;

pub struct Arguments {
	pub ip_addr: IpAddr,
	pub threads: u16,
}

impl Arguments {
	pub fn build(
		mut args: impl Iterator<Item = String>
	) -> Result<Arguments, &'static str> {
		// skip program name
		args.next();

		let first = match args.next() {
			Some(arg) => arg,
			None => return Err("Not enough arguments"),
		};

		if let Ok(ip_addr) = IpAddr::from_str(&first) {
			return Ok(Arguments {
				ip_addr,
				threads: 4, // default value
			});
		}

		let flag = first.clone();

		// If first arg is a flag like -h or -j
		match flag.as_str() {
			"-h" | "-help" => {
				println!("Usage:");
				println!("  my_app <IP>                        | default 4 threads");
				println!("  my_app -j <threads> <IP>           | custom thread count");
				println!("  my_app -h | -help                  | show this help message");
				Err("help")
			}

			"-j" => {
				let threads = match args.next() {
					Some(t) => match t.parse::<u16>() {
						Ok(n) => n,
						Err(_) => return Err("Failed to parse thread number \n Thread count must be a number between 1 and 65535"),
					},
					None => return Err("Missing thread count"),
				};

				let ip_addr = match args.next() {
					Some(ip) => match IpAddr::from_str(&ip) {
						Ok(ip) => ip,
						Err(_) => return Err("Invalid Ip address"),
					},
					None => return Err("Missing IP address"),
				};

				Ok(Arguments {
					ip_addr,
					threads
				})
			}

			_ => Err("Invalid syntax"),
		}
	}
}

pub fn scan(
	tx: Sender<u16>,
	start_port: u16,
	ip_addr: IpAddr,
	num_threads: u16,
) {
	let mut port = start_port + 1;

	loop {
		match TcpStream::connect((ip_addr, port)) {
			Ok(_) => {
				print!("🟢");
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

pub fn run(
	arguments: Arguments
) -> Result<(), Box<dyn Error>>{
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

	Ok(())
}