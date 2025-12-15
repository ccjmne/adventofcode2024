// https://adventofcode.com/2025/day/9
import { readFile } from 'node:fs/promises'
const __dirname = import.meta.url.substring('file://'.length, import.meta.url.lastIndexOf('/'))
const data = String(await readFile(__dirname + '/2025-9.txt')).trim()
  .split(/\n/).map(line => line.split(/,/).map(Number))

const [X, Y] = [data.map(([x]) => x), data.map(([, y]) => y)].map(arr => uniq(arr).toSorted((a, b) => a - b))
const mini = data.map(([x, y]) => [X.indexOf(x), Y.indexOf(y)] as [number, number])
console.log(Math.max(...mini.flatMap((a, i, d) => d.slice(i).map(b => area([a, b])))))

const edges = mini.map((d, i, dd) => [d, dd[(i + 1) % dd.length]])
  .filter(([[x0], [x1]]) => x0 === x1) // vertical edges only, arbitrarily
  .map(([[x, y0], [, y1]]) => [x, Math.min(y0, y1), Math.max(y0, y1), y0 > y1] as [number, number, number, boolean])
  .toSorted(([a], [b]) => a - b)

console.log(Math.max(...mini.flatMap((l, i, d) => d.slice(i + 1).map(r => [l, r])).filter(valid).map(area)))

function valid([[x0, y0], [x1, y1]]) {
  if (x0 > x1 || y0 > y1) return valid([[Math.min(x0, x1), Math.min(y0, y1)], [Math.max(x0, x1), Math.max(y0, y1)]])
  for (let x = x0; x <= x1; ++x) for (let y = y0; y <= y1; ++y) if (!inside(x, y)) return false
  return true
}

function inside(x: number, y: number) {
  const M = inside.prototype.M ??= new Map<string, number>()
  const k = `${x}:${y}`
  // whether rightmost leftward edge is going *up*
  return (M.has(k) ? M : M.set(k, edges.findLast(([xx, y0, y1]) => xx <= x && y >= y0 && y <= y1)?.[3])).get(k)
}

function area([[x0, y0], [x1, y1]]) {
  return (Math.abs(X[x1] - X[x0]) + 1) * (Math.abs(Y[y1] - Y[y0]) + 1)
}

function uniq(arr: number[]) {
  return Object.keys(Object.fromEntries(arr.map(k => [k]))).map(Number)
}
