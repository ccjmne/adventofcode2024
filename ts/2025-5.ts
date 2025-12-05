// https://adventofcode.com/2025/day/5
import { readFile } from 'node:fs/promises'
const __dirname = import.meta.url.substring('file://'.length, import.meta.url.lastIndexOf('/'))
let [ranges, ids] = String(await readFile(__dirname + '/2025-5.txt')).trim().split(/\n\n/)
const R = ranges.split(/\n/).map(r => r.split(/-/).map(Number))

console.log(ids.split(/\n/).map(Number)
  .reduce((count, id) => count + R.some(([lo, hi]) => id >= lo && id <= hi, 0), 0))

const idx = Object.keys(R.flat().reduce((m, idx) => Object.assign(m, { [idx]: 0 }), {})).map(Number).sort((a, b) => a - b)
const mem = Array.from({ length: idx.length - 1 }, () => 0)
R.forEach(([lo, hi]) => range(idx.indexOf(lo), idx.indexOf(hi)).forEach(i => mem[i] = 1))
console.log(mem.reduce((total, on, i) => total + (!on || idx[i + 1] - idx[i]), 1))

function range(lo: number, hi: number) {
  return Array.from({ length: hi - lo }, (_, i) => lo + i)
}
