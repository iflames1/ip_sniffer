# 🔍 ip_sniffer

A simple multithreaded CLI port scanner built in Rust.
Scans a given IP address to detect open TCP ports — fast and efficient using threads.

## ⚙️ Features

-   Multithreaded port scanning (user-defined threads)
-   Scans the full TCP port range (1 to 65535)
-   CLI flags for help and thread control
-   Fast and lightweight

## 🚀 Usage

### Run with default settings (4 threads):

```bash
cargo run -- <IP_ADDRESS>
```

Example:

```bash
cargo run -- 192.168.1.1
```

### Run with custom thread count:

```bash
cargo run -- -j <THREADS> <IP_ADDRESS>
```

Example:

```bash
cargo run -- -j 100 192.168.1.1
```

### Show help:

```bash
cargo run -- -h
```

or

```bash
cargo run -- -help
```

## 🧠 How It Works

The scanner:

-   Spawns multiple threads
-   Each thread takes a slice of ports and tries to connect to them
-   If a connection is successful, it reports that port as open

All open ports are printed after the scan is complete.

## 🛠️ Built With

-   [Rust](https://www.rust-lang.org/)
-   Standard library concurrency (`std::thread`, `std::sync::mpsc`)
-   Sockets via `std::net::TcpStream`

## 📁 Project Structure

```
src/
├── main.rs       # Entry point
└── lib.rs
```

---

Happy scanning! 🚀
