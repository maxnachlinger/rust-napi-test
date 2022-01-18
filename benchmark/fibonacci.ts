export const fibonacciJS = (n: number) => {
  let [a, b] = [0, 1]
  while (--n > 0) {
    ;[a, b] = [b, a + b]
  }
  return b
}
