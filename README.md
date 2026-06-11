# xredis

**A Redis-compatible TCP key-value server built in Rust.**

Engineered from the ground up to explore the internals of networked storage systems — with a focus on correctness, zero-lock concurrency, and idiomatic async Rust.

-----

## Architecture

```
Client (TCP)
     │
     ▼
┌─────────────────┐
│ Connection Task │  — one per client, handles framing & parsing
└────────┬────────┘
         │ mpsc channel
         ▼
┌─────────────────┐
│   Worker Actor  │  — single-threaded, owns all state
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ In-Memory Store │  — HashMap<String, Bytes>
└─────────────────┘
```

The design avoids shared mutable state entirely. Each connection task communicates with a single worker actor via message-passing channels — no `Mutex`, no `RwLock`, no data races by construction.

-----

## Features

|Command|Status     |Description                 |
|-------|-----------|----------------------------|
|`SET`  |✅ Supported|Store a key-value pair      |
|`GET`  |✅ Supported|Retrieve a value by key     |
|`DEL`  |✅ Supported  |Delete one or more keys     |
|`TTL`  |✅ Supported  |Key expiry with time-to-live|

-----

## Design Decisions

### Actor-model concurrency

Rather than sharing state across threads with locks, a single worker task owns the in-memory store and receives commands via `tokio::sync::mpsc`. This eliminates an entire class of concurrency bugs at the architecture level.

### Zero shared mutable state

Connection tasks are fully independent. They parse incoming bytes, construct command messages, and send them to the worker — then await a response over a oneshot channel. No data is shared between tasks.

### Buffered stream parsing

Incoming TCP streams are read through `BufReader` with explicit frame boundaries, making the parser resilient to partial reads and backpressure.

### Explicit error handling

Errors propagate via `Result` at every layer. No panics in the hot path. Connection-level errors are isolated and logged without affecting other clients.

-----

## Getting Started

### Prerequisites

- Rust `1.75+`
- Cargo

### Run the server

```bash
git clone https://github.com/Ali127Dev/xredis.git
cd xredis
cargo run
```

Server starts on `127.0.0.1:6379` by default.

### Connect with redis-cli

```bash
redis-cli -p 6379

127.0.0.1:6379> SET name ali
OK

127.0.0.1:6379> GET name
"ali"
```

-----

## Roadmap

- [x] TCP listener with async connection handling
- [x] Actor-model worker with message-passing
- [x] `SET` command
- [x] `GET` command
- [x] `DEL` command
- [x] `TTL` / `EXPIRE` with background expiry sweep
- [ ] Benchmarks (`wrk` / custom harness)
- [ ] Persistence layer (append-only log)
- [ ] RESP2 protocol full compliance

-----

## Tech Stack

|Crate                 |Purpose                     |
|----------------------|----------------------------|
|`tokio`               |Async runtime, TCP, channels|
|`tokio::sync::mpsc`   |Actor message queue         |
|`tokio::sync::oneshot`|Per-request response channel|
|`bytes`               |Zero-copy byte buffer       |

-----

## Why Rust?

This project targets the internals that most backend engineers never touch:

- How a TCP server handles thousands of concurrent connections without threads-per-connection
- How actor-model architectures eliminate lock contention at the design level
- How Rust’s ownership system enforces correct concurrency at compile time — not at runtime

Built by a backend engineer who works in Go professionally and is learning Rust by building real systems, not tutorials.

-----

## License

MIT
