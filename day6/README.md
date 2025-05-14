# 🚀 Day 6 - Learning Rust

## output

```bash
BMI Calculator
Please enter your weight in kilograms (kg):
78
Please enter you height in meters (m):
1.7
your BMI is: 26.99
BMI category: Overweight
```

## output API call

```bash
curl -X POST http://localhost:3000/bmi   -H "Content-Type: application/json"   -d '{"weight": 80, "height": 1.7}'

{"bmi":27.68166089965398,"category":"Overweight"}
```

## web api

- `axum` is the web framework.
- it consits:
  - `Json`: to extract/return JSON
  - `post`: to define POST routes
  - `Router`: to set up our endpoints

---

```rust
use serde::{Deserialize, Serialize};
```

- Used to define the IP and port the server will run on (like `127.0.0.1:3000`).

---

### Define input JSON structure

```rust
#[derive(Deserialize)]
struct BmiRequest {
    weight: f64,
    height: f64,
}
```

```bash
{ "weight": 60, "height": 1.7 }
```


