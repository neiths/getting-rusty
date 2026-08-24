# simple_server

`simple_server` is a small Rust project that teaches how to build a very basic web server.

It is made for Rust beginners who already know the basic syntax and want to see how Rust works in a real project.

This project has **two important parts**:

- `src/bin/main.rs` — the program entry point
- `src/lib.rs` — your custom reusable code

## What this project is for

This project helps you learn:

- how a Rust binary and library work together
- how a server can handle requests
- how to use threads in Rust
- how to share work across multiple worker threads
- how to organize code into `main.rs` and `lib.rs`

## Project structure

This project usually looks like this:

```text
simple_server
├── Cargo.toml
├── bin
|   ├── main.rs
└── src
    └── lib.rs
```

## What `main.rs` does

`main.rs` is the start of the program.

It usually:

- starts the server
- accepts requests
- uses the code from `lib.rs`
- sends work to the thread pool

## What `lib.rs` does

`lib.rs` contains your custom library code.

In this project, `lib.rs` defines a **thread pool**.

A thread pool is a group of worker threads that can do jobs for you instead of creating a new thread every time.

This is useful for servers because a server may need to handle many requests.

---

## Your custom code: `ThreadPool`

Your `lib.rs` defines this main type:

```rust
pub struct ThreadPool
```

This is a reusable object that manages worker threads.

### Why a thread pool is useful

Without a thread pool, a server might create a new thread for every request.

That can be expensive.

With a thread pool:

- the threads are created once
- jobs are sent to workers
- workers do the work one by one

This is more efficient and easier to manage.

---

## How `ThreadPool` works

Your `ThreadPool` has two main parts:

```rust
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}
```

### `workers`

This is a list of worker threads.

Each worker waits for a job and then runs it.

### `sender`

This sends jobs to the workers.

It uses Rust’s channel system:

- `mpsc` means **multiple producer, single consumer**
- many parts of the program can send jobs
- one receiver gets the jobs and passes them to workers

---

## What a `Job` is

You defined:

```rust
type Job = Box<dyn FnOnce() + Send + 'static>;
```

This means a job is:

- a closure or function
- can be called only once
- safe to move to another thread
- valid for the whole program lifetime

In simple words: a job is some work the thread pool can run later.

---

## The `new` function

```rust
pub fn new(size: usize) -> ThreadPool
```

This creates a new thread pool.

### What it does

1. checks that `size` is greater than zero
2. creates a channel
3. wraps the receiver in `Arc<Mutex<...>>`
4. creates the worker threads
5. stores the workers and sender

### Important beginner note

This line:

```rust
assert!(size > 0);
```

means the thread pool must have at least one worker.

If `size` is zero, the program will panic.

---

## Why `Arc<Mutex<...>>` is used

This part can look confusing at first:

```rust
let receiver = std::sync::Arc::new(std::sync::Mutex::new(receiver));
```

### `Arc`
`Arc` means **shared ownership**.

It lets multiple workers share the same receiver.

### `Mutex`
`Mutex` means only one thread can use the receiver at a time.

This is needed because many workers cannot safely read from the same receiver at the same time.

In simple words:

- `Arc` lets many workers share it
- `Mutex` keeps access safe

---

## The `execute` function

```rust
pub fn execute<F>(&self, f: F)
```

This sends a job into the thread pool.

### What it does

1. takes a function or closure
2. puts it into a `Box`
3. sends it through the channel
4. a worker receives it and runs it

### Example idea

You can think of it like this:

- `main.rs` says: “here is some work”
- `ThreadPool` sends it to a worker
- the worker does the work

---

## The `Worker` type

Your code also defines:

```rust
struct Worker {
    id: usize,
    thread: Option<std::thread::JoinHandle<()>>,
}
```

A worker is one thread in the pool.

### `id`
This is the worker number.

It helps identify which worker is doing the job.

### `thread`
This stores the thread handle.

A thread handle lets Rust keep track of the spawned thread.

---

## What a worker does

A worker is created with:

```rust
fn new(id: usize, receiver: std::sync::Arc<std::sync::Mutex<mpsc::Receiver<Job>>>) -> Worker
```

### Worker behavior

Each worker:

1. waits for a job
2. locks the receiver
3. gets the next job
4. prints a message
5. runs the job

This part runs in a loop:

```rust
loop {
    let job = receiver.lock().unwrap().recv().unwrap();
    println!("Worker {} got a job; executing.", id);
    job();
}
```

### In simple words

The worker never stops waiting.

It keeps listening for new jobs and runs them one by one.

---

## What this code teaches

This library is a good beginner example because it shows:

- how to build a reusable Rust module
- how to use structs and methods
- how to use threads
- how to use channels
- how to share data safely between threads
- how to write code that can be used by `main.rs`

---

## Requirements

To run this project, you need:

- Rust
- Cargo

Check your setup:

```bash
rustc --version
cargo --version
```

---

## How to run

Go to the project folder:

```bash
cd /home/thienhb/Workspace/getting-rusty/learning-2026/simple_server
```

Run the project:

```bash
cargo run
```

---

## How to learn from this project

If you are new to Rust server code, read it in this order:

1. `src/main.rs`
2. `src/lib.rs`
3. `ThreadPool::new`
4. `ThreadPool::execute`
5. `Worker::new`

That order makes the code easier to understand.

---

## Good beginner exercises

Try changing these things:

- change the number of workers
- print more messages from each worker
- change the job type
- experiment with different requests in `main.rs`
- see how the thread pool handles multiple tasks

---

## Summary

This project is a simple Rust server example with a custom thread pool in `src/lib.rs`.

It is useful for learning:

- Rust project structure
- reusable library code
- threads
- jobs
- safe shared access with `Arc` and `Mutex`

It is meant for learning, not for production use.