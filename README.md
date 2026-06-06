# 🚀 Redis-Inspired Server in Rust

A small project built to explore Rust from a backend engineering
perspective.

The focus is not on cloning Redis feature-for-feature, but on
understanding the building blocks behind modern network services:

-   ⚡ Async I/O with Tokio
-   🌐 TCP networking
-   📬 Message passing
-   🎭 Actor-style architecture
-   🔒 Ownership-driven concurrency
-   🧠 Stream processing and framing

------------------------------------------------------------------------

## ✨ Architecture

``` text
Client
   │
   ▼
Connection Task
   │
   ▼
Message Queue
   │
   ▼
Worker
   │
   ▼
In-Memory State
```

The project avoids shared mutable state and instead relies on message
passing between independent tasks.

------------------------------------------------------------------------

## 🦀 Why Rust?

This project is mainly a playground for learning:

-   Ownership & Borrowing
-   Lifetimes
-   Async/Await
-   Channels
-   Error Handling
-   Concurrency without data races

------------------------------------------------------------------------

## 🏗️ Design Principles

-   Single responsibility per module
-   Clear ownership boundaries
-   Explicit communication paths
-   Simple and readable code
-   Learning-oriented implementation

------------------------------------------------------------------------

## 🔍 Topics Explored

-   TCP streams
-   Request/response workflows
-   Buffered stream parsing
-   Backpressure concepts
-   Actor model patterns
-   Event-driven systems

------------------------------------------------------------------------

## ▶️ Running

``` bash
cargo run
```

------------------------------------------------------------------------

## 🎯 Goal

Build a deeper understanding of how networked services work internally
while becoming more fluent in Rust's programming model.

------------------------------------------------------------------------

Made with ☕, curiosity, and a lot of compiler errors.
