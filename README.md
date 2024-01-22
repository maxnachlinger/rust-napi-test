# `rust-napi-test`

## Getting started

- [install Rust](https://www.rust-lang.org/tools/install) 
- `yarn global add @napi-rs/cli` or `npm i -g @napi-rs/cli`
- `yarn` or `npm i`
- `yarn build` or `npm run build`

## Benchmarks

- `yarn bench` or `npm run bench`

## Results on my machine

```shell
node -v
v16.14.0
```

### Fibonacci Benchmarks
| | ops/sec | # of runs |
| :---------- | ----------: | ----------: |
| js fib 10 | 55,360,895 | 90 |
| wasm fib 10 | 27,436,213 | 90 |
| js fib 20 | 28,259,782 | 90 |
| wasm fib 20 | 25,873,549 | 86 |
| js fib 40 | 14,761,155 | 91 |
| wasm fib 40 | 23,580,599 | 91 |
| js fib 100 | 5,711,025 | 93 |
| wasm fib 100 | 14,390,518 | 90 |
