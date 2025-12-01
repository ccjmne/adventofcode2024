import { readFile } from 'fs/promises'
const __dirname = import.meta.url.substring('file://'.length, import.meta.url.lastIndexOf('/'))

const data = String(await readFile(__dirname + '/4.txt')).trim()
  .split(/\n/).map(line => [...line])

console.log(
  data.flatMap((row, r) => row.map((x, c) => x === 'X' && [r, c]).filter(match => !!match))
    .map(([r, c]) => xmas([r, c], data))
    .reduce((sum, x) => sum + x))

console.log(
  data.flatMap((row, r) => row.map((x, c) => x === 'A' && [r, c]).filter(match => !!match))
    .map(([r, c]) => x_mas([r, c], data))
    .filter(match => match).length)

function xmas([r, c]: [number, number], map: string[][]): number {
  return [
    [map[r]?.[c - 1], map[r]?.[c - 2], map[r]?.[c - 3]],
    [map[r + 1]?.[c], map[r + 2]?.[c], map[r + 3]?.[c]],
    [map[r - 1]?.[c], map[r - 2]?.[c], map[r - 3]?.[c]],
    [map[r]?.[c + 1], map[r]?.[c + 2], map[r]?.[c + 3]],
    [map[r - 1]?.[c + 1], map[r - 2]?.[c + 2], map[r - 3]?.[c + 3]],
    [map[r + 1]?.[c + 1], map[r + 2]?.[c + 2], map[r + 3]?.[c + 3]],
    [map[r + 1]?.[c - 1], map[r + 2]?.[c - 2], map[r + 3]?.[c - 3]],
    [map[r - 1]?.[c - 1], map[r - 2]?.[c - 2], map[r - 3]?.[c - 3]],
  ].filter(letters => letters.join('') === 'MAS').length
}

function x_mas([r, c]: [number, number], map: string[][]): boolean {
  return [
    [map[r - 1]?.[c + 1], map[r + 1]?.[c - 1]],
    [map[r + 1]?.[c + 1], map[r - 1]?.[c - 1]],
    [map[r + 1]?.[c - 1], map[r - 1]?.[c + 1]],
    [map[r - 1]?.[c - 1], map[r + 1]?.[c + 1]],
  ].filter(letters => letters.join('') === 'MS').length === 2
}
