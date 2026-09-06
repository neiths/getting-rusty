# IP Sniffer (Port Scanner in Rust)

A fast, lightweight, and concurrent TCP port scanner written in Rust using only the standard library (`std`).

This project scans all 65,535 TCP ports for a given IPv4 or IPv6 address by distributing the workload across multiple threads using message passing channels.

---

## Features

- **Zero External Dependencies**: Pure Rust standard library.
- **Multi-threaded Concurrency**: Configurable thread pool dividing port ranges evenly.
- **Connection Timeouts**: Prevents threads from hanging indefinitely on closed or firewalled ports.
- **Safe Integer Arithmetic**: Uses `checked_add` to eliminate port overflow and off-by-one errors.
- **Robust Argument Parsing**: Clean error messages, help screens, and non-zero exit codes on failure without panicking.
- **Unit Tested**: Includes automated tests for argument parsing and port coverage logic.

---

## Usage

### Build the project

```bash
cargo build --release
```

### Run with default threads (4 threads)

```bash
cargo run -- 127.0.0.1
```

### Run with custom thread count (e.g., 100 threads)

```bash
cargo run -- -j 100 127.0.0.1
# or
cargo run -- --threads 100 127.0.0.1
```

### Display Help

```bash
cargo run -- -h
# or
cargo run -- --help
```

### Run Tests

```bash
cargo test
```

---

## Example Output

```text
Scanning 127.0.0.1 using 100 threads (timeout: 500ms)...
.......
1716 is open
5355 is open
33695 is open
43503 is open
44412 is open
46613 is open
54296 is open
Scan complete. Found 7 open port(s).
```

---

## Key Optimizations & Improvements

This implementation optimizes the classic beginner tutorial port sniffer while keeping the code clean and easy to understand:

| Area | Original Tutorial Implementation | Optimized Implementation |
| :--- | :--- | :--- |
| **Connection Timeout** | `TcpStream::connect` blocks for OS default (30–120s per port). Filtered ports cause extreme slowdowns. | `TcpStream::connect_timeout` with 500ms limit keeps scanning fast and responsive. |
| **Port Coverage** | Condition `(MAX - port) <= num_threads` caused thread to exit prematurely, skipping port 65535 and higher ports. | `port.checked_add(num_threads)` guarantees all 65,535 ports are visited without overflow. |
| **Argument Safety** | Passing incomplete arguments like `-j 100` crashed with index out of bounds panic. | Safe pattern matching on `args.len()` prevents bounds errors and reports clean CLI errors. |
| **Help Handling** | `flag.contains("-h")` had operator precedence issues and mixed printing inside parsing. | Extracted `print_help`, handles `-h` / `--help`, and returns standard exit codes (`0` for help, `1` for error). |
| **Dead Code / Warnings** | Stored an unused `flag: String` field in `Arguments`, producing clippy warnings. | Removed redundant fields and unnecessary `return` statements; zero compiler and clippy warnings. |
| **Formatting** | Dots (`.`) printed on the same line without a trailing newline before results. | Added newline spacing and a summary counter showing total open ports found. |

---

## How It Works

### 1. Work Distribution (Striding)

Instead of allocating contiguous chunks of ports to each thread (which could cause one thread to get stuck on a block of slow ports), each thread strides through the port space:

- **Thread 0**: Ports `1, 1 + T, 1 + 2T, ...`
- **Thread 1**: Ports `2, 2 + T, 2 + 2T, ...`
- **Thread `k`**: Ports `k+1, (k+1) + T, ...`

```rust
let mut port: u16 = start_port + 1;
loop {
    let socket = SocketAddr::new(addr, port);
    if TcpStream::connect_timeout(&socket, TIMEOUT).is_ok() {
        print!(".");
        let _ = io::stdout().flush();
        let _ = tx.send(port);
    }

    match port.checked_add(num_threads) {
        Some(next) => port = next,
        None => break,
    }
}
```

### 2. Multi-Producer, Single-Consumer Channel (`mpsc`)

```
[ Worker Thread 0 ] ---\
[ Worker Thread 1 ] ----+---> [ mpsc::channel ] ---> [ Main Thread (Receiver) ]
[ Worker Thread N ] ---/
```

- Each worker receives a clone of `tx` (`Sender<u16>`).
- When an open port is discovered, `tx.send(port)` sends it to `rx`.
- `drop(tx)` is called in the main thread so that when all worker threads finish and drop their senders, the receiver automatically stops waiting.
- Results are collected into a `Vec<u16>`, sorted with `sort_unstable()`, and printed.

---

## Rust Concepts Practiced

- **Ownership & Move Semantics**: Moving cloned `Sender` handles into thread closures (`move || { ... }`).
- **Channels & Concurrency**: Using `std::sync::mpsc` for safe lock-free communication between threads.
- **Checked Arithmetic**: Using `checked_add` to handle integer overflow gracefully.
- **Error Handling**: Using `Result`, `map_err`, and the `?` operator for clean parsing.
- **Unit Testing**: Testing edge cases using `#[cfg(test)]` and `cargo test`.
