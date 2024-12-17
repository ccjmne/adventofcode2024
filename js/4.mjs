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

/**
 * @param {[number, number]} coords
 * @param {number[][]} map
 * @returns {number}
 */
function xmas([r, c], map) {
  let res = 0
  res += [map[r]?.[c - 1], map[r]?.[c - 2], map[r]?.[c - 3]].join('') === 'MAS'
  res += [map[r + 1]?.[c], map[r + 2]?.[c], map[r + 3]?.[c]].join('') === 'MAS'
  res += [map[r - 1]?.[c], map[r - 2]?.[c], map[r - 3]?.[c]].join('') === 'MAS'
  res += [map[r]?.[c + 1], map[r]?.[c + 2], map[r]?.[c + 3]].join('') === 'MAS'
  res += [map[r - 1]?.[c + 1], map[r - 2]?.[c + 2], map[r - 3]?.[c + 3]].join('') === 'MAS'
  res += [map[r + 1]?.[c + 1], map[r + 2]?.[c + 2], map[r + 3]?.[c + 3]].join('') === 'MAS'
  res += [map[r + 1]?.[c - 1], map[r + 2]?.[c - 2], map[r + 3]?.[c - 3]].join('') === 'MAS'
  res += [map[r - 1]?.[c - 1], map[r - 2]?.[c - 2], map[r - 3]?.[c - 3]].join('') === 'MAS'
  return res
}

/**
 * @param {[number, number]} coords
 * @param {number[][]} map
 * @returns {boolean}
 */
function x_mas([r, c], map) {
  let res = 0
  res += [map[r - 1]?.[c + 1], map[r + 1]?.[c - 1]].join('') === 'MS'
  res += [map[r + 1]?.[c + 1], map[r - 1]?.[c - 1]].join('') === 'MS'
  res += [map[r + 1]?.[c - 1], map[r - 1]?.[c + 1]].join('') === 'MS'
  res += [map[r - 1]?.[c - 1], map[r + 1]?.[c + 1]].join('') === 'MS'
  return res == 2
}
