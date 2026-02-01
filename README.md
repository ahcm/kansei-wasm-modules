# kansei-wasm-modules
WASM modules for Kansei. Install with `kansei wasm install <name>`.

## example
Example module with numeric exports:
- `fastx_version()` -> i32
- `fastx_add_i64(a, b)` -> i64
- `fastx_mul_f64(a, b)` -> f64
- `fastx_pow_f64(a, b)` -> f64

Build target: `wasm32-wasip1`.

Target install:
```
rustup target add wasm32-wasip1
```
