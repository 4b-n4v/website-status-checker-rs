# Website Status Checker Project in Rust

By: Angelo Brian Navilon
### Optional Features Added:
- Optional --retries <N> (default = 0) additional attempts after a failure, with a 100 ms pause between attempts.

### Build Instructions:
```sh
git clone https://github.com/4b-n4v/website-status-checker-rs.git
cd website-status-checker-rs
cargo build --release
```
### Example Use:
```sh
# From root directory:
./target/release/web-stat-check --file urls.txt --workers 4 --timeout 3 --retries 5
```
### Tree Structure
```txt
.
├── src
│   ├── checker.rs
│   ├── cli.rs
│   ├── main.rs
│   ├── status.rs
│   └── worker.rs
├── target
│   ├── release
│   │   ├── build
│   │   ├── examples
│   │   ├── incremental
│   │   ├── web-stat-check       <----- THIS IS THE BINAAAAAARY
│   │   └── web-stat-check.d
├── Cargo.lock
├── Cargo.toml
├── README.md
├── status.json                  <----- Not included; produced by binary.
└── urls.txt
```
