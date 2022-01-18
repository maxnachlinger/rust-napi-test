import test from 'ava'

import { fibonacciNative } from '../index'

test('sync function from native code', (t) => {
  const fixture = 42
  t.is(fibonacciNative(fixture), fixture + 100)
})
