# Embeddings

This project demonstrates how token embeddings are represented and used in a transformer-style model.

## What it does

The code wraps Burn's embedding layer and shows how integer token IDs are mapped to dense vector representations.

It includes:

- an `EmbeddingLayerConfig` for defining vocabulary size and model dimension
- a simple embedding module that looks up token vectors
- a sample forward pass using token IDs such as `464` and `3797`

## Why this matters

Embeddings are the first step in encoding text for neural models. Each token is converted into a vector, and later layers combine those vectors with positional information and attention.

## Files

- `src/main.rs` contains the embedding setup and example usage

## Run it

```bash
cargo run
```

## Dependencies

- `burn`
- `candle-core`
- `candle-nn`

## Example

The program creates an embedding lookup for a vocabulary of 50,257 entries and a hidden size of 768, then checks the output shape:

```text
Input shape:  [1, 2]
Output shape: [1, 2, 768]
```
