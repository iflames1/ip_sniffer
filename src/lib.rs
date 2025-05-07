use std::{net::IpAddr, str::FromStr};

pub struct Arguments {
	pub flag: String,
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
				flag: String::new(),
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
					flag,
					ip_addr,
					threads
				})
			}

			_ => Err("Invalid syntax"),
		}
	}
}