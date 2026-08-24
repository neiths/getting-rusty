# RoPE

This project explores Rotary Positional Embedding (RoPE), a positional encoding technique commonly used in transformer-based language models.

## What it does

RoPE rotates pairs of embedding dimensions by an angle based on token position. This allows the model to preserve relative positional information without adding extra learned position vectors.

In this implementation, the code:

- builds inverse frequency values from the model dimension
- precomputes cosine and sine tables for the maximum sequence length
- applies a half-rotation to query or key tensors
- demonstrates the transformation with a synthetic attention tensor

## Key idea

For each token position, RoPE rotates feature pairs such as:

- $(x_1, x_2)$ becomes $(x_1 \cos\theta - x_2 \sin\theta,\ x_1 \sin\theta + x_2 \cos\theta)$

This is useful in attention mechanisms because relative positions can be represented through rotation.

## Files

- `src/main.rs` contains the RoPE implementation and a small example run

## Run it

```bash
cargo run
```

## Dependencies

- `burn` for tensor operations and model-building utilities

## Notes

This is a learning-focused implementation meant to illustrate the mechanics of RoPE rather than a production-ready model component.
