// https://adventofcode.com/2025/day/1
import { readFile } from 'fs/promises'
const __dirname = import.meta.url.substring('file://'.length, import.meta.url.lastIndexOf('/'))
const data = String(await readFile(__dirname + '/2025-1.txt')).trim()
const movs = data.split(/\n/).map(move => move.substring(1) * (move[0] === 'L' ? -1 : 1))

console.log(movs.reduce(
  ({ dial, ctr }, d) => ({ dial: dial + d, ctr: ctr + !mod100(dial + d) }),
  { dial: 50, ctr: 0 }
).ctr)

console.log(movs.reduce(({ dial, ctr }, d) => {
  let over = Math.floor(Math.abs(d / 100))
  let next = mod100(dial + d)
  return { dial: next, ctr: ctr + over + (next === 0 || dial !== 0 && Math.sign(d) !== Math.sign(next - dial)) }
}, { dial: 50, ctr: 0 }).ctr)

function mod100(x: number) {
  return (x % 100 + 100) % 100
}
