use std::{env, process};

use ip_sniffer::Arguments;


fn main() {
    let arguments = Arguments::build(env::args()).unwrap_or_else(
        |err| {
            if err.contains("help") {
                process::exit(0);
            } else {
                eprintln!("Error: {err}");
                process::exit(1);
            }
        }
    );

    if let Err(err) = ip_sniffer::run(arguments) {
        eprintln!("Application Error: {err}");
        process::exit(2);
    }
}
