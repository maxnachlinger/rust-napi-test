#![deny(clippy::all)]

use napi_derive::napi;

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
