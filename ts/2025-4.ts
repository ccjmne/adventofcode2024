// https://adventofcode.com/2025/day/4
import { readFile } from 'node:fs/promises'
const __dirname = import.meta.url.substring('file://'.length, import.meta.url.lastIndexOf('/'))
let data = String(await readFile(__dirname + '/2025-4.txt')).trim()
  .split(/\n/).map(row => row.split(''))

// Don't call stabilise for part 1
console.log(stabilise(0, () => (data = data.map((row, r) => row.map((col, c) => {
  if (col === '.' || col === 'x') return col
  return around(r, c, data) >= 4 ? '@' : 'x'
}))).join('').match(/x/g).length))

function around(r: number, c: number, data: string[][]) {
  return [
    [r - 1, c - 1], [r - 1, c], [r - 1, c + 1],
    [r    , c - 1],             [r    , c + 1],
    [r + 1, c - 1], [r + 1, c], [r + 1, c + 1],
  ].reduce((count, [r, c]) => count + (data[r]?.[c] === '@'), 0)
}

function stabilise(prev: number, next: () => number): number {
  const cur = next()
  return cur === prev ? cur : stabilise(cur, next)
}
