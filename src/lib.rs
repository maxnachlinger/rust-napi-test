#![deny(clippy::all)]

use napi_derive::napi;

#[cfg(all(
  any(windows, unix),
  target_arch = "x86_64",
  not(target_env = "musl"),
  not(debug_assertions)
))]
#[global_allocator]
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[napi]
fn fibonacci_native(n: i64) -> i64 {
  let mut x: i64 = 1;
  let mut y: i64 = 1;
  let mut temp: i64;

  for _ in 1..n {
    temp = x;
    x += y;
    y = temp
  }

  y
}
