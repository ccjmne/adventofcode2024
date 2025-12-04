// https://adventofcode.com/2025/day/3
import { readFile } from 'node:fs/promises'
const __dirname = import.meta.url.substring('file://'.length, import.meta.url.lastIndexOf('/'))
const data = String(await readFile(__dirname + '/2025-3.txt')).trim()
  // use range(0, 2) for part 1
  .split(/\n/).map(bank => Number(range(0, 12).reverse().reduce(({ I, l }, i) => {
    const { v, at } = range(I + 1, bank.length - i).reduce((o, j) => Number(bank[j]) > o.v ? ({ v: bank[j], at: j }) : o, { v: -1, at: -1 })
    return { I: at, l: l + v }
  }, { I: -1, l: '' }).l))

console.log(data.reduce((sum, n) => sum + n, 0))

function range(lo: number, hi: number) {
  return Array.from({ length: hi - lo }).map((_, i) => lo + i)
}
