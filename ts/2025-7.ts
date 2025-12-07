// https://adventofcode.com/2025/day/7
import { readFile } from 'node:fs/promises'
const __dirname = import.meta.url.substring('file://'.length, import.meta.url.lastIndexOf('/'))
const data = String(await readFile(__dirname + '/2025-7.txt')).trim().split(/\n/)
const start = data.shift().indexOf('S')

console.log(data.reduce(([beams, splits], row) => {
  const [rbeams, rsplits] = beams.reduce(
    ([b, s], beam) => row[beam] === '^' ? [[...b, beam - 1, beam + 1], s + 1] : [[...b, beam], s],
    [[], 0])
  return [uniq(rbeams), splits + rsplits]
}, [[start], 0] as [number[], number])[1])

console.log((function timelines(beam: number, depth: number) {
  const M = timelines.prototype.M ??= new Map<string, number>()
  const k = `${beam}:${depth}`
  return M.has(k) ? M.get(k) : M.set(k, (function() {
    if (depth >= data.length) return 1
    return (data[depth][beam] === '^')
      ? timelines(beam - 1, depth + 1) + timelines(beam + 1, depth + 1)
      : timelines(beam, depth + 1)
  }())).get(k)
}(start, 0)))

function uniq(arr: number[]) {
  return Object.keys(Object.fromEntries(arr.map(a => [a]))).map(Number)
}
