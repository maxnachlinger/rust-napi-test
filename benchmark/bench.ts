import b from 'benny'
import { Summary } from 'benny/lib/internal/common-types'

import { fibonacciNative } from '../index'

import { fibonacciJS } from './fibonacci'

const testFib = (n: number): Promise<Summary> =>
  b.suite(
    `Fibonacci ${n}`,

    b.add('Rust', () => {
      fibonacciNative(n)
    }),
    b.add('JS', () => {
      fibonacciJS(n)
    }),

    b.cycle(),
    b.complete(),
  )

async function run() {
  await testFib(10)
  await testFib(100)
  await testFib(1000)
}

run().catch((e) => {
  console.error(e)
})
