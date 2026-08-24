# tiktoken-rs

This project is a small Rust exploration of tokenization using the `tiktoken-rs` crate.

## What it does

It wraps OpenAI's BPE-based tokenizer logic and exposes a simple interface for:

- encoding text into token IDs
- decoding token IDs back to text
- handling special tokens such as end-of-text markers
- testing common tokenization cases like normal text, emoji, and long words

## Key features

- `TokenizerConfig` defines the tokenizer name and vocabulary size
- `SimpleTokenizer::new()` creates a tokenizer instance
- `encode()` converts text to `Vec<Rank>` token IDs
- `decode()` converts token IDs back to a string
- `vocab_size()` exposes the configured vocabulary size

## Files

- `src/main.rs` contains the tokenizer wrapper and several example checks

## Run it

```bash
cargo run
```

## Dependencies

- `tiktoken-rs`

## Example behavior

The sample program tests:

- basic English text
- the EOS token
- a rare word
- emoji and Unicode input

This makes it useful as a small tokenizer reference for LLM-style text preprocessing.
