# `rust-napi-test`

## Getting started

- [install Rust](https://www.rust-lang.org/tools/install)
- `npm i -g @napi-rs/cli`
- `pnpm i`
- `pnpm build`

## Benchmarks

- `pnpm bench`

## Results on my machine

```shell
node -v
v23.1.0
```

```
Running "Fibonacci 10" suite...
Progress: 100%

  Rust:
    41 010 955 ops/s, ±1.09%   | slowest, 53.48% slower

  JS:
    88 158 246 ops/s, ±2.96%   | fastest

Finished 2 cases!
  Fastest: JS
  Slowest: Rust
Running "Fibonacci 40" suite...
Progress: 100%

  Rust:
    28 304 850 ops/s, ±2.81%   | slowest, 14.29% slower

  JS:
    33 024 586 ops/s, ±0.75%   | fastest

Finished 2 cases!
  Fastest: JS
  Slowest: Rust
Running "Fibonacci 50" suite...
Progress: 100%

  Rust:
    23 631 275 ops/s, ±0.80%   | fastest

  JS:
    17 136 342 ops/s, ±0.47%   | slowest, 27.48% slower

Finished 2 cases!
  Fastest: Rust
  Slowest: JS
Running "Fibonacci 100" suite...
Progress: 100%

  Rust:
    16 039 578 ops/s, ±2.94%   | fastest

  JS:
    8 337 312 ops/s, ±1.42%    | slowest, 48.02% slower

Finished 2 cases!
  Fastest: Rust
  Slowest: JS
```
