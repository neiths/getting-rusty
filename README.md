# getting-rusty

This repository tracks my learning journey with the Rust programming language, with a focus on exploring core concepts and practical implementations in AI and systems programming.

## Repository structure

### Core learning projects

- [23-days-learning-rust](23-days-learning-rust) — day-by-day Rust exercises and mini projects
- [exercises](exercises) — standalone Rust problem-solving exercises and small apps
- [embeddings](embeddings) — token embedding example built with Burn
- [RoPE](RoPE) — rotary positional embedding implementation for transformer-style attention
- [tiktoken-rs](tiktoken-rs) — tokenizer wrapper using the `tiktoken-rs` crate
- [hecto](hecto) — terminal text editor in Rust with Unicode grapheme support

## New implementations

### 1. tiktoken-rs

This project demonstrates tokenization with a BPE tokenizer, which is a core step in modern LLM pipelines.

It covers:

- encoding text into token IDs
- decoding IDs back to text
- handling special tokens
- testing real-world examples like Unicode and long words

See [tiktoken-rs/README.md](tiktoken-rs/README.md) for details.

### 2. embeddings

This project shows how token IDs are converted into dense vectors using an embedding layer.

It is useful for understanding:

- vocabulary lookup
- hidden dimension sizing
- input preparation for transformers

See [embeddings/README.md](embeddings/README.md) for details.

### 3. RoPE

This project implements rotary positional embeddings, a technique used to inject positional information into transformer attention.

It focuses on:

- position-aware rotations of feature pairs
- cosine/sine positional tables
- attention-facing tensor transformations

See [RoPE/README.md](RoPE/README.md) for details.

### 4. hecto

This project is a terminal text editor built in Rust with raw mode terminal handling, 2D scrolling, and full Unicode grapheme cluster awareness.

See [hecto/README.md](hecto/README.md) for details.

## Quick run commands

```bash
cargo run --manifest-path tiktoken-rs/Cargo.toml
cargo run --manifest-path embeddings/Cargo.toml
cargo run --manifest-path RoPE/Cargo.toml
cargo run --manifest-path hecto/Cargo.toml
```

## Notes

These implementations are intentionally simple and educational. They are designed to help understand the building blocks behind transformer models and token processing in Rust.
