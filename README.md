# `rust-napi-test`

## Getting started

- [install Rust](https://www.rust-lang.org/tools/install) 
- `npm i -g @napi-rs/cli`
- `npm i`
- `npm run build`

## Benchmarks

- `npm run bench`

## Results on my machine

```shell
node -v
v22.7.0
```

```
Running "Fibonacci 10" suite...
Progress: 100%

  Rust:
    41 521 556 ops/s, ±2.94%   | slowest, 53.07% slower

  JS:
    88 470 327 ops/s, ±2.28%   | fastest

Finished 2 cases!
  Fastest: JS
  Slowest: Rust
Running "Fibonacci 20" suite...
Progress: 100%

  Rust:
    38 226 744 ops/s, ±0.82%   | slowest, 34.95% slower

  JS:
    58 761 983 ops/s, ±1.22%   | fastest

Finished 2 cases!
  Fastest: JS
  Slowest: Rust
Running "Fibonacci 40" suite...
Progress: 100%

  Rust:
    30 942 510 ops/s, ±1.32%   | slowest, 6.56% slower

  JS:
    33 113 264 ops/s, ±1.22%   | fastest

Finished 2 cases!
  Fastest: JS
  Slowest: Rust
Running "Fibonacci 100" suite...
Progress: 100%

  Rust:
    18 684 872 ops/s, ±0.44%   | fastest

  JS:
    13 800 095 ops/s, ±0.65%   | slowest, 26.14% slower

Finished 2 cases!
  Fastest: Rust
  Slowest: JS
```