// https://adventofcode.com/2024/day/6
import { readFile } from 'fs/promises'
const __dirname = import.meta.url.substring('file://'.length, import.meta.url.lastIndexOf('/'))
const data = String(await readFile(__dirname + '/6.txt')).trim()

type Dir = 'L' | 'D' | 'U' | 'R' | '?' | '!' // '?' means stuck in a loop, '!' means got out
type Cell = [blocked: boolean, been: string] // `been` contains at most one of each of L, D, U, R
type Guard = [row: number, col: number, dir: Dir]

const map: Cell[][] = data.split(/\n/g).map(line => [...line].map(c => [c === '#', '']))
const start: Guard = [0 | data.indexOf('^') / (map[0].length + 1), data.indexOf('^') % (map[0].length + 1), 'U']

loop(map)
console.log(map.flatMap((row   ) => row.filter(([, been]   ) => !!been))                          .length)
console.log(map.flatMap((row, r) => row.filter(([, been], c) => !!been && loop(block(map, r, c)))).length)

function step(map: Cell[][], [row, col, dir]: Guard): Guard {
  if (map[row][col][1].includes(dir)) return [row, col, '?']; else map[row][col][1] += dir
  const [dr, dc] = { L: [0, -1], U: [-1, 0], D: [+1, 0], R: [0, +1] }[dir]
  const [r, c] = [row + dr, col + dc]
  switch (map[r]?.[c]?.[0]) {
    case undefined: return [row, col, '!']
    case true:      return [row, col, 'URDL'[('URDL'.indexOf(dir) + 1) % 4] as Dir]
    case false:     return [r,   c,   dir]
  }
}

function loop(map: Cell[][], guard: Guard = start): boolean {
  while (!/[!?]/.test(guard[2])) guard = step(map, guard)
  return guard[2] === '?'
}

function block(map: Cell[][], br: number, bc: number): Cell[][] {
  return map.map((row, r) => row.map(([blocked], c) => [blocked || (br === r && bc === c), ''] as Cell))
}
